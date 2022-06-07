
pub fn challenge1() {
   
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

pub fn challenge2() {

    //Fixed XOR
    //Write a function that takes two equal-length buffers and produces their XOR combination.
    //If your function works properly, then when you feed it the string:
    //1c0111001f010100061a024b53535009181c
    //... after hex decoding, and when XOR'd against:
    //686974207468652062756c6c277320657965
    //... should produce:
    //746865206b696420646f6e277420706c6179

    let inp_bytes1 = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
    println!("{:?}", inp_bytes1);    

    let inp_bytes2 = hex::decode("686974207468652062756c6c277320657965").unwrap();
    println!("{:?}", inp_bytes2);

    //check if input are the same length
    assert_eq!(inp_bytes1.len(), inp_bytes2.len());

    let mut z = Vec::new();

    for (x,y) in  inp_bytes1.iter().zip(inp_bytes2.iter()) {
    
            z.push(x ^ y);
                
    }
    print!("{:?}", hex::encode(z));

}