use rust_contentful_normalizer::string_helpers::to_camel_case;

fn main() {  
    let result = to_camel_case("testing-function");
    println!("{}", result)
}
