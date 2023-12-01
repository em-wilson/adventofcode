pub struct PeekableString {
    data: String,
}

impl PeekableString {
    pub fn create_from_string(input: String) -> PeekableString {
        return PeekableString{
            data: input
        }
    }

    pub fn len(&self) -> usize {
        return self.data.len();
    }

    fn peek_string(&self, start: usize, length: usize) -> String {
        return self.data.chars().skip(start).take(length).collect()
    }
    
    pub fn peek_number(&self, key:String, value:String, start: usize) -> Option<String> {
        let peeked_value : String = self.peek_string(start, key.len());
        if peeked_value == key.to_string() {
            return Some(value.to_string());
        }
        return None;
    }
    
}