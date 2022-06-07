extern crate base64;
extern crate hex;

fn main() {
   
    //Set 1  Challenge 1
    //Convert hex to base64
    //The string:
    //49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
    //Should produce:
    //SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t

    let inp_bytes = hex::decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();
    println!("{:?}", inp_bytes);

    let b64 = base64::encode(inp_bytes);
    println!("{}", b64);


    
}