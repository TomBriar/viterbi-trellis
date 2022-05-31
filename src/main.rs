use image::io::Reader as ImageReader;
use image::{GenericImageView, Rgba};
use std::io::{stdin,stdout,Write};
use std::f64;
use std::f64::INFINITY;
// use turbojpeg::compress_image;
// use turbojpeg::Subsamp; 
// use arrayfire::save_image_native;


fn main() {

	let mut cover_object: Vec<i8> = Vec::new(); //cover_object vector 
	let mut full_image: Vec<i8> = Vec::new(); // the entire image vector
	let mut cover_weights: Vec<i8> = Vec::new(); //weights for each cover_object bit
	let img = match ImageReader::open("./dog.jpeg") { // grab image from filename
		Ok(reader) => match reader.decode() {
			Ok(img) => img,
			Err(e) => panic!("{}", e),
		},
		Err(e) => panic!("{}", e),
	};
	for i in 0..img.height() {
		for ii in 0..img.width() {
			let pixel = img.get_pixel(ii, i); //Get each pixel
			let r = pixel[0]; //grab the red byte
			let mut r_byte = format!("{r:b}"); //convert to binary
			for _ in 0..(8-r_byte.len()) { //pad binary to proper 8 bit form
				let filler: String = String::from("0"); //create 0 string
				r_byte = filler+&r_byte //pad binary
			}
			for r in r_byte.chars() { //loop through bits in the binary string
				full_image.push(r.to_string().parse::<i8>().unwrap()) // convert to i8 int and push to full image vector
			}
			let r_lsb = r_byte.pop().expect("Never panic").to_string().parse::<i8>().unwrap(); // pop the lsb convert to i8 int
			cover_object.push(r_lsb); //push to cover_object vector
			cover_weights.push(1); //set weight for all bits to 1
		}
	}

	for i in 1..full_image.len() {
		if ((i+1)%8) == 0 {
			assert_eq!(full_image[i], cover_object[(((i+1)/8)-1) as usize]); //assert the full image contains all image bits while the cover object contains only the lsbs
		}
	}

	let mut s = String::new(); //Create message string
    print!("Please enter some text: "); //Print to console
    let _=stdout().flush(); //new line for console
    stdin().read_line(&mut s).expect("Did not enter a correct string"); //Grab message
    s = s.trim().to_string();

	let mut message = Vec::<i8>::new(); //create message vector
	for m in s.bytes() { //loop through bytes of the message string
		let mut binary = format!("{m:b}"); //convert bytes to bits
		for _ in 0..(8-binary.len()) { //bad binary to proper 8 bit form
			let filler: String = String::from("0"); //create a 0 string
			binary = filler+&binary //pad binary
		}
		for i in binary.chars() { //grab each bit of the binary string
			message.push(i.to_string().parse::<i8>().unwrap()); //parse binary into a i8 int and push to message vector
		}
	}
	print!("message: ");
	for i in 0..message.len() {
		print!("{}, ", message[i]);
	}
	println!(";");
	

	

	while cover_object.len()%message.len() != 0 { //trim cover_object to be a multiple of the message vector
		cover_object.pop(); //pop the end off the cover_object
	}
	let sub_width = cover_object.len()/message.len(); //rate of the encoding or the width of the sub matrix H
    let sub_height: usize = 2; //performance parameter
    let h = 2_i8.pow(sub_height as u32); //2^h
    let mut sub_h: Vec<Vec<i8>> = Vec::new(); //create the sub_h or h_hat vector
    for i in 0..sub_height {
    	sub_h.push(Vec::new());
    	for _ in 0..sub_width {
    		if rand::random() { //randomly push a zero or one
    			sub_h[i].push(1); 
    		} else {
    			sub_h[i].push(0);
    		}
    	} 
    }

    let mut sub_ch: Vec<Vec<i8>> = Vec::new(); //create the column oriented sub_h or h_hat
    for i in 0..sub_width {
    	sub_ch.push(Vec::new());
    	for ii in 0..sub_height {
    		sub_ch[i].push(sub_h[ii][i]);
    	}
    }

    let mut ph_hat: Vec<Vec<i8>> = Vec::new(); //A vector of vectors, the first of which contains the int format for each column of sub_h or h_hat, the remaining vectors contain the trimed columns based on the extended H. 
    for i in 0..sub_height { //The number of trimed column blocks
    	ph_hat.push(Vec::new());
    	for ii in 0..sub_width {
    		ph_hat[i].push(0);
    		for iii in 0..(sub_height-i) { //the number of bits per trimmed column
    			ph_hat[i][ii] += sub_ch[ii][iii]*2_i8.pow(iii as u32); //binary to int
    		}
    	}
    }
    
 
    let ext_height = message.len(); //extended matrix H height
    let ext_width = cover_object.len(); //extended matrix W width
    let b = ext_width/sub_width; //Number of copies of sub_h or h_hat in the extended matrix. Includes trimmed blocks.


    let mut ext_h: Vec<Vec<i8>> = Vec::new(); //extended matrix
    for i in 0..(ext_height) {
    	ext_h.push(Vec::new());
    	for _ in 0..ext_width {
    		ext_h[i].push(0);
    	}
    }
    let mut ext_ch: Vec<Vec<i8>> = Vec::new(); //extended matrix column oriented
    for i in 0..(ext_width) {
    	ext_ch.push(Vec::new());
    	for _ in 0..ext_height {
    		ext_ch[i].push(0);
    	}
    }

    let mut row = 0;
    let mut column = 0;
    'B: for _ in 0..(ext_width/sub_width) { //Builds the extended matrix
    	for ii in 0..sub_height {
    		for iii in 0..sub_width {
    			if (row+ii >= ext_height) || (column+iii >= ext_width) {
    				break 'B
    			}
    			ext_h[row+ii][column+iii] = sub_h[ii][iii];
    		}
    	}
    	row += 1;
		column = column+sub_width;
    }
    
    for i in 0..ext_h[0].len() { //Builds the column oriented extended matrix
    	for ii in 0..ext_h.len() {
    		ext_ch[i][ii] = ext_h[ii][i];
    	}
    }

	fn matrix_multi(s: &mut Vec<i8>, x: &mut Vec<i8>, ch: &Vec<Vec<i8>>, ext_height: usize) { //multiplys a vector of length equal to that of the cover object against the extended matrix. The result is a syndrom the length of the message.
		for _ in 0..ext_height {
			s.push(0);
		}
		for i in 0..ch.len() {
			for ii in 0..ch[0].len() {
				s[i] = (s[i]+((x[ii]*ch[i][ii])%2))%2;
			}
		}
		for i in 0..ext_height {
			s[i] = s[i]%2;
		}
	}



	let mut path: Vec<Vec<i8>> = Vec::new(); //path vector of vectors contains a vector of each state for each column.
	for i in 0..cover_object.len() {
		path.push(Vec::new());
		for _ in 0..h {
			path[i].push(-2);
		}
	}
	let mut wght: Vec<f64> = Vec::new(); //contains the cost per path
	wght.push(0.0);
	for _ in 1..h {
		wght.push(INFINITY);
	}
	let mut y: Vec<i8> = Vec::new(); //stego cover object
	for _ in 0..cover_object.len() {
		y.push(0);
	}
	let mut indx = 0;
	let mut indm = 0;
	for _ in 1..((b+1) as usize) { //For each copy of sub_h in ext_h
		for j in 0..((sub_width) as usize) { //for each column
			let mut newwght: Vec<f64> = Vec::new();
			for _ in 0..h {
				newwght.push(INFINITY);
			}
			for k in 0..(h as usize) { //for each state 
				let mut phindex = 0; 
				if (indm+sub_height) > b { //Decides if the current column is a trimed version of sub_h or h_hat
					phindex = (indm+sub_height)-b;
				}
				let w0 = wght[k] + ((cover_object[indx]*cover_weights[indx]) as f64); //weight of not adding the current column of sub_h or h_hat
				let w1 = wght[((k as i8)^ph_hat[phindex][(j%sub_width) as usize]) as usize] + ((((1+cover_object[indx])%2)*cover_weights[indx]) as f64); //weight of adding the current column of sub_h or h_hat
				path[indx][k] = if w1 < w0 { //recordes the available paths for this state
					1
				} else {
					0
				};
				newwght[k] = w0.min(w1); //decides if adding or not addingh the column of h_hat was cheeper
			}
			indx += 1;
			wght = newwght;
		}
		for j in 0..h/2 {
			wght[j as usize] = wght[((2*j) + message[indm]) as usize]; // squashes the weights by half taking either the even node or the odd node based on the message bit
		}
		for j in h/2..h {
			wght[j as usize] = INFINITY; //zeros out the second half after the squash
		}
		indm += 1;
	}
	let embeding_cost = wght[0]; 
	println!("embeding cost = {}", embeding_cost);

	let mut state: i8 = message[(indm-1) as usize]; //current state of the trellis
	for _ie in 1..((b+1) as usize) { //for each copy of sub_h or h_hat
		indm -= 1;
		// let _i = b-ie; // To go backwards
		for je in 1..((sub_width+1) as usize) { //for each column
			indx -= 1;
			let j = sub_width-je; // To go backwards
			y[indx] = path[indx][state as usize]; //set the stego object bit for this state
			let mut phindex = 0;
			if (indm+sub_height) > b { //decides if we need to use a trimed copy of h_hat or sub_h
				phindex = (indm+sub_height)-b; 
			}
			state = state^((y[indx]*ph_hat[phindex][(j%sub_width) as usize])); //updates the state based on cheepest choice
		}
		if indm == 0 {
			break
		}
		state = (2*state + message[indm-1 as usize]) % h; //updates the state to account for the pruning
	}

	let mut syndrom = Vec::new();
	matrix_multi(&mut syndrom, &mut y,  &ext_h, ext_height);
	for i in 0..syndrom.len() {
		println!("s{} = {}", i, syndrom[i]);
	}
	assert!(syndrom == message); //assert the encoding was done properl

	for i in 1..full_image.len() {
		if (((i+1)%8) == 0) && ((((i+1)/8)-1) < y.len()) {
			full_image[i] = y[(((i+1)/8)-1) as usize];
			// assert_eq!(full_image[i], cover_object[(((i+1)/8)-1) as usize]); //assert the full image contains all image bits while the cover object contains only the lsbs
		}
	}


	let mut newimg = img.into_rgba8();
	let mut index = 0;
	for i in 0..newimg.height() {
		for ii in 0..newimg.width() {
			let pixel = newimg.get_pixel(ii, i);
			if index < y.len() {
				let r = pixel[0];
				let mut r_byte = format!("{r:b}");
				for _ in 0..(8-r_byte.len()) {
					let filler: String = String::from("0");
					r_byte = filler+&r_byte
				}
				r_byte.pop();
				r_byte = r_byte+&y[index].to_string();
				let new_pixel = Rgba([u8::from_str_radix(&r_byte, 2).unwrap(), pixel[1], pixel[2], pixel[3]]);
				newimg.put_pixel(ii, i, new_pixel);
			}
			index += 1;
		}
	}
	// let jpeg_data = turbojpeg::compress_image(&newimg, 100, Subsamp::None).expect("failed to compress_image");
	newimg.save("./dog2.jpeg").expect("Faild to save image");

	let readimg = match ImageReader::open("./dog2.jpeg") {
		Ok(reader) => match reader.decode() {
			Ok(img) => img,
			Err(e) => panic!("{}", e),
		},
		Err(e) => panic!("{}", e),
	};
	let mut stego_ob = Vec::new();
	for i in 0..readimg.height() {
		for ii in 0..readimg.width() {
			let pixel = readimg.get_pixel(ii, i);
			let r = pixel[0];
			let mut r_byte = format!("{r:b}");
			for _ in 0..(8-r_byte.len()) {
				let filler: String = String::from("0");
				r_byte = filler+&r_byte
			}
			let r_lsb = r_byte.pop().expect("Never panic").to_string().parse::<i8>().unwrap();
			stego_ob.push(r_lsb);
		}
	}
	println!("-----------");
	let mut syndrom = Vec::new();
	matrix_multi(&mut syndrom, &mut stego_ob,  &ext_h, ext_height);
	for i in 0..syndrom.len() {
		println!("s{} = {}", i, syndrom[i]);
	}
	println!("-----------");
	let mut syndrom = Vec::new();
	matrix_multi(&mut syndrom, &mut cover_object,  &ext_h, ext_height);
	for i in 0..syndrom.len() {
		println!("s{} = {}", i, syndrom[i]);
	}
	
	// assert!(stego_ob == stego_object);



}
