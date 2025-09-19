use image::{Rgb, ImageReader, RgbImage};
use std::env;
use progression::Bar;

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len()<5 {
        eprintln!("Usage: majorityimage in1 in2 in3 ... out");
        return;
    }
    let mut images = Vec::new();
    for i in 1..args.len()-1 {
        println!("opening image {}",args[i]);
        images.push(ImageReader::open(&args[i]).unwrap().decode().unwrap().into_rgb8());
    }

    let width=images[0].width();
    let height=images[0].height();

    for i in 0..images.len() {
        if (images[i].width()!=width) || (images[i].height()!=height) {
            eprintln!("input image number {} is not the same size", i);
            return;
        }
    }
    let mut out_image : RgbImage = RgbImage::new(width, height);

    let mut pxlversions = Vec::new();
    let mut pxlerrors = Vec::new();
    let mut avgpixel = Rgb::<u32>::from([0,0,0]);
    for _ in 0..images.len() {
        pxlversions.push(Rgb::<u8>::from([0,0,0]));
        pxlerrors.push(0 as u32);
    }


    let bar:Option<Bar> = if env::var("NO_BAR").is_ok() { None } else { Some(Bar::new(height as u64, progression::Config { prefix: "(items) ", ..progression::Config::cargo() } ) ) };
    for y in 0..height {
        for x in 0..width {
            for i in 0..images.len() {
                pxlversions[i]=*images[i].get_pixel(x, y);
                avgpixel[0]+=pxlversions[i][0] as u32;
                avgpixel[1]+=pxlversions[i][1] as u32;
                avgpixel[2]+=pxlversions[i][2] as u32;
            }
            avgpixel[0]/=images.len() as u32 + 1;
            avgpixel[1]/=images.len() as u32 + 1;
            avgpixel[2]/=images.len() as u32 + 1;
            for i in 0..images.len() {
                pxlerrors[i]=
                    (pxlversions[i][0].abs_diff(avgpixel[0] as u8) as u32)+
                    (pxlversions[i][1].abs_diff(avgpixel[1] as u8) as u32)+
                    (pxlversions[i][2].abs_diff(avgpixel[2] as u8) as u32);
            }
            let out_pxl = out_image.get_pixel_mut(x, y);
            let minepixi = indexofmin(&pxlerrors);
            out_pxl[0] = pxlversions[minepixi][0];
            out_pxl[1] = pxlversions[minepixi][1];
            out_pxl[2] = pxlversions[minepixi][2];
        }
        if let Some(ref bar_some) = bar { bar_some.inc(1); }
    }

    if let Some(bar_some) = bar { bar_some.finish(); }
    out_image.save(&args[args.len()-1]).unwrap();
}

fn indexofmin(vector: &Vec<u32>) -> usize {
    let mut min = vector[0];
    let mut minindex = 0;
    for i in 1..vector.len() {
        if vector[i]<min {
            min=vector[i];
            minindex=i;
        }
    }
    minindex
}

        // for x in 0..width {
        //     for i in 0..images.len() {
        //         pxlversions[i][0]=images[i].get_pixel(x, y)[0];
        //         pxlversions[i][1]=images[i].get_pixel(x, y)[1];
        //         pxlversions[i][2]=images[i].get_pixel(x, y)[2];
        //     }
        //     for i in 0..images.len() {
        //         for j in (i+0)..images.len() {
        //             pxlerrors[i]=
        //                 (pxlversions[i][0].abs_diff(pxlversions[j][0]) as u32)+
        //                 (pxlversions[i][1].abs_diff(pxlversions[j][1]) as u32)+
        //                 (pxlversions[i][2].abs_diff(pxlversions[j][2]) as u32);
        //         }
        //     }
        //     let out_pxl = out_image.get_pixel_mut(x, y);
        //     let minepixi = indexofmin(&pxlerrors);
        // }
