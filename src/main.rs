use image::io::Reader as ImageReader;
use image::{GenericImageView, DynamicImage, Rgb, RgbImage};
use std::io::{stdin,stdout,Write};
use std::f64;
use std::f64::INFINITY;
use jpeg_encoder::{Encoder, ColorType};
use jpeg_encoder::{ImageBuffer, JpegColorType, rgb_to_ycbcr};


// use img;
// use turbojpeg::image::Rgba;
// use turbojpeg::Subsamp; 
// use arrayfire::save_image_native;
// use image::codecs::jpeg::JpegEncoder;


fn main() {
	fn print_type_of<T>(_: &T) {
	    println!("{}", std::any::type_name::<T>())
	}
	pub struct RgbImageBuffer {
	    image: RgbImage,
	}

	impl ImageBuffer for RgbImageBuffer {
	    fn get_jpeg_color_type(&self) -> JpegColorType {
	        // Rgb images are encoded as YCbCr in JFIF files
	        JpegColorType::Ycbcr
	    }

	    fn width(&self) -> u16 {
	        self.image.width() as u16
	    }

	    fn height(&self) -> u16 {
	        self.image.height() as u16
	    }

	    fn fill_buffers(&self, y: u16, buffers: &mut [Vec<u8>; 4]){
	        for x in 0..self.width() {
	            let pixel = self.image.get_pixel(x as u32 ,y as u32);

	            let (y,cb,cr) = rgb_to_ycbcr(pixel[0], pixel[1], pixel[2]);

	            // For YCbCr the 4th buffer is not used
	            buffers[0].push(y);
	            buffers[1].push(cb);
	            buffers[2].push(cr);
	        }
	    }
	}

	let mut cover_object: Vec<u64> = Vec::new(); //cover_object vector 
	let mut full_image: Vec<u8> = Vec::new(); // the entire image vector
	let mut cover_weights: Vec<u64> = Vec::new(); //weights for each cover_object bit
	let jpeg_data = std::fs::read("./dog.jpeg").expect("failed to read image");

	let img = match ImageReader::open("./dog.jpeg") { // grab image from filename
		Ok(reader) => match reader.decode() {
			Ok(img) => img,
			Err(e) => panic!("{}", e),
		},
		Err(e) => panic!("{}", e),
	};
	// let img: img::RgbImage = turbojpeg::decompress_image(&jpeg_data).expect("failed to decompress");
	'E: for i in 0..img.height() {
		for ii in 0..img.width() {
			let pixel = img.get_pixel(i, ii); //Get each pixel
			let r = pixel[0]; //grab the red byte
			let mut r_byte = format!("{r:b}"); //convert to binary
			for _ in 0..(8-r_byte.len()) { //pad binary to proper 8 bit form
				let filler: String = String::from("0"); //create 0 string
				r_byte = filler+&r_byte //pad binary
			}
			print!("{}-{}={}, ", i, ii, r);
			for r in r_byte.chars() { //loop through bits in the binary string
				full_image.push(r.to_string().parse::<u64>().unwrap() as u8) // convert to u64 int and push to full image vector
			}
			let r_lsb = r_byte.pop().expect("Never panic").to_string().parse::<u64>().unwrap(); // pop the lsb convert to u64 int
			cover_object.push(r_lsb); //push to cover_object vector
			cover_weights.push(1); //set weight for all bits to 1
			if (cover_object.len() >= 16) {
				break 'E
			}
		}
	}
	println!(";");

	for i in 1..full_image.len() {
		if ((i+1)%8) == 0 {
			assert_eq!(full_image[i], cover_object[(((i+1)/8)-1) as usize] as u8); //assert the full image contains all image bits while the cover object contains only the lsbs
		}
	}
	// let encoder = JpegEncoder::new_with_quality(full_image, 100);
	// img.save("./dog2.png").expect("Faild to save image");
	let mut rgb_image: image::RgbImage = img.to_rgb8();
	
	let i = 0;
	let ii = 0;	

	let pixel = img.get_pixel(i,ii);
	let r = pixel[0];
	let mut r_byte = format!("{r:b}");
	for _ in 0..(8-r_byte.len()) {
		let filler: String = String::from("0");
		r_byte = filler+&r_byte
	}
	r_byte.pop();
	r_byte = r_byte+&"1".to_string();
	// print!("{}-{}={}, ", i, ii, u8::from_str_radix(&r_byte, 2).unwrap());
	let new_pixel = Rgb([u8::from_str_radix(&r_byte, 2).unwrap(), pixel[1], pixel[2]]);
	rgb_image.put_pixel(i, ii, new_pixel);
	let pixel = rgb_image.get_pixel(i, ii); //Get each pixel
	let r = pixel[0]; //grab the red byte
	println!("{}, h", r);
	let mut r_byte = format!("{r:b}"); //convert to binary
	for _ in 0..(8-r_byte.len()) { //pad binary to proper 8 bit form
		let filler: String = String::from("0"); //create 0 string
		r_byte = filler+&r_byte //pad binary
	}
	print!("{}-{}={}, ", i, ii, r);

	println!(";");

	let mut encoder = Encoder::new_file("dog2.jpeg", 100).expect("failed to open for write");
	encoder.set_sampling_factor(jpeg_encoder::SamplingFactor::R_4_1_1);
	// encoder.set_progressive_scans(30);
	// print_type_of(&encoder.sampling_factor());
	let image_buffer: RgbImageBuffer = RgbImageBuffer {image: rgb_image};
	encoder.encode_image(image_buffer);

	let img2 = match ImageReader::open("./dog2.jpeg") { // grab image from filename
		Ok(reader) => match reader.decode() {
			Ok(img) => img,
			Err(e) => panic!("{}", e),
		},
		Err(e) => panic!("{}", e),
	};
	let mut stego_ob: Vec<u64> = Vec::new();
	'E: for i in 0..img2.height() {
		for ii in 0..img2.width() {
			let pixel = img2.get_pixel(i, ii); //Get each pixel
			let r = pixel[0]; //grab the red byte
			print!("{}-{}={}, ", i, ii, r);
			let mut r_byte = format!("{r:b}"); //convert to binary
			for _ in 0..(8-r_byte.len()) { //pad binary to proper 8 bit form
				let filler: String = String::from("0"); //create 0 string
				r_byte = filler+&r_byte //pad binary
			}
			let r_lsb = r_byte.pop().expect("Never panic").to_string().parse::<u64>().unwrap(); // pop the lsb convert to u64 int
			// println!("test");
			stego_ob.push(r_lsb); //push to cover_object vector
			if (stego_ob.len() >= 16) {
				break 'E
			}
		}
	}
	println!(";");
	print!("cover_object: ");
	for i in 0..cover_object.len() {
		print!("{}, ", cover_object[i]);
	}
	println!(";");
	print!("stego_ob    : ");
	for i in 0..stego_ob.len() {
		print!("{}, ", stego_ob[i]);
	}
	println!(";");
	assert!(cover_object == stego_ob);


	// let mut s = String::new(); //Create message string
 //    print!("Please enter some text: "); //Print to console
 //    let _=stdout().flush(); //new line for console
 //    stdin().read_line(&mut s).expect("Did not enter a correct string"); //Grab message
 //    s = s.trim().to_string();

	// let mut message = Vec::<u64>::new(); //create message vector
	// for m in s.bytes() { //loop through bytes of the message string
	// 	let mut binary = format!("{m:b}"); //convert bytes to bits
	// 	for _ in 0..(8-binary.len()) { //bad binary to proper 8 bit form
	// 		let filler: String = String::from("0"); //create a 0 string
	// 		binary = filler+&binary //pad binary
	// 	}
	// 	for i in binary.chars() { //grab each bit of the binary string
	// 		message.push(i.to_string().parse::<u64>().unwrap()); //parse binary into a u64 int and push to message vector
	// 	}
	// }
	// print!("message: ");
	// for i in 0..message.len() {
	// 	print!("{}, ", message[i]);
	// }
	// println!(";");
	

	

	// while cover_object.len()%message.len() != 0 { //trim cover_object to be a multiple of the message vector
	// 	cover_object.pop(); //pop the end off the cover_object
	// }
	// println!("cover_object.len() = {}", cover_object.len());
	// println!("message.len() = {}", message.len());
	// let sub_width = cover_object.len()/message.len(); //rate of the encoding or the width of the sub matrix H
 //    let sub_height: usize = 4; //performance parameter
 //    let h = 2_u64.pow(sub_height as u32); //2^h
 //    let mut sub_h: Vec<Vec<u64>> = Vec::new(); //create the sub_h or h_hat vector
 //    for i in 0..sub_height {
 //    	sub_h.push(Vec::new());
 //    	for _ in 0..sub_width {
 //    		if rand::random() { //randomly push a zero or one
 //    			sub_h[i].push(1); 
 //    		} else {
 //    			sub_h[i].push(0);
 //    		}
 //    	} 
 //    }
 //    // println!("sub_h");
 //    // for i in 0..sub_h.len() {
 //    // 	for ii in 0..sub_h[i].len() {
 //    // 		print!("{}, ", sub_h[i][ii]);
 //    // 	}
 //    // 	println!(";");
 //    // }

 //    let mut sub_ch: Vec<Vec<u64>> = Vec::new(); //create the column oriented sub_h or h_hat
 //    for i in 0..sub_width {
 //    	sub_ch.push(Vec::new());
 //    	for ii in 0..sub_height {
 //    		sub_ch[i].push(sub_h[ii][i]);
 //    	}
 //    }

 //    let mut ph_hat: Vec<Vec<u64>> = Vec::new(); //A vector of vectors, the first of which contains the int format for each column of sub_h or h_hat, the remaining vectors contain the trimed columns based on the extended H. 
 //    for i in 0..sub_height { //The number of trimed column blocks
 //    	ph_hat.push(Vec::new());
 //    	for ii in 0..sub_width {
 //    		ph_hat[i].push(0);
 //    		for iii in 0..(sub_height-i) { //the number of bits per trimmed column
 //    			ph_hat[i][ii] += sub_ch[ii][iii]*2_u64.pow(iii as u32); //binary to int
 //    		}
 //    	}
 //    }
 //    // for i in 0..sub_h[0].len() {
 //    // 	print!("{}, ", sub_h[0][i]);
 //    // }
    
 
 //    let ext_height = message.len(); //extended matrix H height
 //    let ext_width = cover_object.len(); //extended matrix W width
 //    println!("sub_width = {}", sub_width);
 //    let b = ext_width/sub_width; //Number of copies of sub_h or h_hat in the extended matrix. Includes trimmed blocks.


 //    let mut ext_h: Vec<Vec<u64>> = Vec::new(); //extended matrix
 //    for i in 0..(ext_height) {
 //    	ext_h.push(Vec::new());
 //    	for _ in 0..ext_width {
 //    		ext_h[i].push(0);
 //    	}
 //    }

 //    let mut ext_ch: Vec<Vec<u64>> = Vec::new(); //extended matrix column oriented
 //    for i in 0..(ext_width) {
 //    	ext_ch.push(Vec::new());
 //    	for _ in 0..ext_height {
 //    		ext_ch[i].push(0);
 //    	}
 //    }

 //    let mut row = 0;
 //    let mut column = 0;
 //    'B: for _ in 0..(ext_width/sub_width) { //Builds the extended matrix
 //    	'H: for ii in 0..sub_height {
 //    		for iii in 0..sub_width {
 //    			if row+ii >= ext_height {
 //    				break 'H
 //    			}
 //    			if column+iii >= ext_width {
 //    				break 'B
 //    			}
 //    			ext_h[row+ii][column+iii] = sub_h[ii][iii];
 //    		}
 //    	}
 //    	row += 1;
	// 	column = column+sub_width;
 //    }
    
 //    for i in 0..ext_h[0].len() { //Builds the column oriented extended matrix
 //    	for ii in 0..ext_h.len() {
 //    		ext_ch[i][ii] = ext_h[ii][i];
 //    	}
 //    }


 //    //   println!("ext_h");
 //    // for i in 0..ext_h.len() {
 //    // 	for ii in 0..ext_h[i].len() {
 //    // 		print!("{}, ", ext_h[i][ii]);
 //    // 	}
 //    // 	println!(";");
 //    // }

	// fn matrix_multi(s: &mut Vec<u64>, x: &mut Vec<u64>, ch: &Vec<Vec<u64>>, ext_height: usize) { //multiplys a vector of length equal to that of the cover object against the extended matrix. The result is a syndrom the length of the message.
	// 	for _ in 0..ext_height {
	// 		s.push(0);
	// 	}
	// 	for i in 0..ch.len() {
	// 		for ii in 0..ch[0].len() {
	// 			// if (i == (ch.len()-1)) {
	// 			// 	println!("ch[{}][{}] = {}, x[{}] = {}", i, ii, ch[i][ii], ii, x[ii]);
	// 			// }
	// 			s[i] = (s[i]+((x[ii]*ch[i][ii])%2))%2;
	// 		}
	// 	}
	// 	for i in 0..ext_height {
	// 		s[i] = s[i]%2;
	// 	}
	// }



	// let mut path: Vec<Vec<u64>> = Vec::new(); //path vector of vectors contains a vector of each state for each column.
	// for i in 0..cover_object.len() {
	// 	path.push(Vec::new());
	// 	for _ in 0..h {
	// 		path[i].push(0);
	// 	}
	// }
	// let mut wght: Vec<f64> = Vec::new(); //contains the cost per path
	// wght.push(0.0);
	// for _ in 1..h {
	// 	wght.push(INFINITY);
	// }
	// let mut y: Vec<u64> = Vec::new(); //stego cover object
	// for _ in 0..cover_object.len() {
	// 	y.push(0);
	// }
	// let mut indx = 0;
	// let mut indm = 0;
	// // println!("b = {}", b);
	// for _ in 1..((b+1) as usize) { //For each copy of sub_h in ext_h
	// 	for j in 0..((sub_width) as usize) { //for each column
	// 		let mut newwght: Vec<f64> = Vec::new();
	// 		for _ in 0..h {
	// 			newwght.push(INFINITY);
	// 		}
	// 		for k in 0..(h as usize) { //for each state 
	// 			let mut phindex = 0; 
	// 			if (indm+sub_height) > b { //Decides if the current column is a trimed version of sub_h or h_hat
	// 				phindex = (indm+sub_height)-b;
	// 			}
	// 			let w0 = wght[k] + ((cover_object[indx]*cover_weights[indx]) as f64); //weight of not adding the current column of sub_h or h_hat
	// 			let w1 = wght[((k as u64)^ph_hat[phindex][(j%sub_width) as usize]) as usize] + ((((1+cover_object[indx])%2)*cover_weights[indx]) as f64); //weight of adding the current column of sub_h or h_hat
	// 			path[indx][k] = if w1 < w0 { //recordes the available paths for this state
	// 				1
	// 			} else {
	// 				0
	// 			};
	// 			// println!("j = {}, w0 = {}, w1 = {}, p[{}][{}] = {}", j, w0, w1, indx, k, path[indx][k]);
	// 			newwght[k] = w0.min(w1); //decides if adding or not addingh the column of h_hat was cheeper
	// 		}
	// 		indx += 1;
	// 		wght = newwght;
	// 	}
	// 	for j in 0..h/2 {
	// 		// println!("{}, {}", indm, message[indm]);
	// 		wght[j as usize] = wght[((2*j) + message[indm]) as usize]; // squashes the weights by half taking either the even node or the odd node based on the message bit
	// 	}
	// 	for j in h/2..h {
	// 		wght[j as usize] = INFINITY; //zeros out the second half after the squash
	// 	}
	// 	indm += 1;
	// }
	// let embeding_cost = wght[0]; 
	// println!("embeding cost = {}", embeding_cost);

	// let mut state: u64 = message[(indm-1) as usize]; //current state of the trellis //message[(indm-1) as usize]
	// for _ie in 1..((b+1) as usize) { //for each copy of sub_h or h_hat
	// 	indm -= 1;
	// 	// let _i = b-ie; // To go backwards
	// 	for je in 1..((sub_width+1) as usize) { //for each column
	// 		indx -= 1;
	// 		let j = sub_width-je; // To go backwards
	// 		y[indx] = path[indx][state as usize]; //set the stego object bit for this state
	// 		let mut phindex = 0;
	// 		if (indm+sub_height) > b { //decides if we need to use a trimed copy of h_hat or sub_h
	// 			phindex = (indm+sub_height)-b; 
	// 		}
	// 		state = state^((y[indx]*ph_hat[phindex][(j%sub_width) as usize])); //updates the state based on cheepest choice
	// 	}
	// 	if indm == 0 {
	// 		break
	// 	}
	// 	state = (2*state + message[indm-1 as usize]) % h; //updates the state to account for the pruning
	// }
	// // for i in 0..cover_object.len() {
	// // 	print!("{}, ", cover_object[i]);
	// // }
	// // println!(";");
	// // for i in 0..y.len() {
	// // 	print!("{}, ", y[i]);
	// // }
	// // println!(";");


	// let mut syndrom = Vec::new();
	// matrix_multi(&mut syndrom, &mut y,  &ext_h, ext_height);
	// // print!("syndrom: ");
	// // for i in 0..syndrom.len() {
	// // 	print!("{}, ", syndrom[i]);
	// // }
	// // println!(";");
	
	// assert!(syndrom == message); //assert the encoding was done properl

	// for i in 1..full_image.len() {
	// 	if (((i+1)%8) == 0) && ((((i+1)/8)-1) < y.len()) {
	// 		full_image[i] = y[(((i+1)/8)-1) as usize];
	// 		// assert_eq!(full_image[i], cover_object[(((i+1)/8)-1) as usize]); //assert the full image contains all image bits while the cover object contains only the lsbs
	// 	}
	// }


	// let mut newimg = img.into_rgba8();
	// let mut index = 0;
	// for i in 0..newimg.height() {
	// 	for ii in 0..newimg.width() {
	// 		let pixel = newimg.get_pixel(i, ii);
	// 		if index < y.len() {
	// 			let r = pixel[0];
	// 			let mut r_byte = format!("{r:b}");
	// 			for _ in 0..(8-r_byte.len()) {
	// 				let filler: String = String::from("0");
	// 				r_byte = filler+&r_byte
	// 			}
	// 			r_byte.pop();
	// 			r_byte = r_byte+&y[index].to_string();
	// 			let new_pixel = Rgba([u8::from_str_radix(&r_byte, 2).unwrap(), pixel[1], pixel[2], pixel[3]]);
	// 			newimg.put_pixel(ii, i, new_pixel);
	// 		}
	// 		index += 1;
	// 	}
	// }
	// // let jpeg_data = turbojpeg::compress_image(&newimg, 100, Subsamp::None).expect("failed to compress_image");
	// // newimg.save("./dog2.jpeg").expect("Faild to save image");

	// let readimg = match ImageReader::open("./dog2.jpeg") {
	// 	Ok(reader) => match reader.decode() {
	// 		Ok(img) => img,
	// 		Err(e) => panic!("{}", e),
	// 	},
	// 	Err(e) => panic!("{}", e),
	// };
	// let mut stego_ob = Vec::new();
	// for i in 0..readimg.height() {
	// 	for ii in 0..readimg.width() {
	// 		let pixel = readimg.get_pixel(i, ii);
	// 		let r = pixel[0];
	// 		let mut r_byte = format!("{r:b}");
	// 		for _ in 0..(8-r_byte.len()) {
	// 			let filler: String = String::from("0");
	// 			r_byte = filler+&r_byte
	// 		}
	// 		let r_lsb = r_byte.pop().expect("Never panic").to_string().parse::<u64>().unwrap();
	// 		stego_ob.push(r_lsb);
	// 	}
	// }
	// println!("-----------");
	// let mut syndrom = Vec::new();
	// matrix_multi(&mut syndrom, &mut stego_ob,  &ext_h, ext_height);
	// for i in 0..syndrom.len() {
	// 	println!("s{} = {}", i, syndrom[i]);
	// }
	// // println!("-----------");
	// // let mut syndrom = Vec::new();
	// // matrix_multi(&mut syndrom, &mut cover_object,  &ext_h, ext_height);
	// // for i in 0..syndrom.len() {
	// // 	println!("s{} = {}", i, syndrom[i]);
	// // }
	
	// // assert!(stego_ob == stego_object);



}
