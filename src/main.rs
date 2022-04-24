#![feature(portable_simd)]
use core::simd::f32x4;

#[allow(unused)]
fn main() 
{

    let input = [2.0f32;129];
    let output = [2.0f32;129];
    //let (prefix_in, middle_in, suffix_in) = input.as_simd::<SIMD_LENGTH>();
    //let (prefix_out, middle_out, suffix_out) = output.as_simd::<SIMD_LENGTH>();
    
    let (prefix_in, middle_in, suffix_in) = unsafe {input.align_to::<f32x4>()};
    let (prefix_out, middle_out, suffix_out) = unsafe {output.align_to::<f32x4>()};
    //let gain_vec = f32x4::splat(gain);
    //middle_out.iter_mut().zip(middle_in).for_each(|(to, from)|{*to = from * gain_vec});
//
    //prefix_out.iter_mut().zip(prefix_in).for_each(|(o, i)|{*o = i*gain});
    //suffix_out.iter_mut().zip(suffix_in).for_each(|(o, i)|{*o = i*gain});

    println!("{:?}", output);
}
