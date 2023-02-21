pub struct Equation {
    text: String,
    pub tokens: Vec<Token>,
}
impl Equation {
	pub fn new(text: String) -> Equation {
		return Equation{
			text,
			tokens: Vec::new(),
		}
	}
    pub fn parse(&mut self) {
        let mut buf: String = String::new();
        let mut last_num = false;
        for c in self.text.chars() {
            if c.is_ascii_digit() {
                buf.push(c);
                last_num = true;
            } else if last_num == true {
                self.tokens
                    .push(Token::Constant(buf.parse::<f32>().unwrap()));
                buf.clear();
            }
			if c.is_ascii_punctuation() {
				self.tokens.push(Token::Operator(c))
			}
        }
        if buf.is_empty() == false {
            self.tokens
                .push(Token::Constant(buf.parse::<f32>().unwrap()));
        }
    }
}
#[derive(Debug)]
pub enum Token {
    Constant(f32),
    Operator(char),
}
