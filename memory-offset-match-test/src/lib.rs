#[cfg(test)]
mod tests {
    use super::*;
    use memory_offset_match::RegexMagic;

    #[derive(RegexMagic)]
    #[byte_pattern("13 37 (yourPointer: ????) 66 66")]
    struct YourPattern {
        #[bytes_from(le)]
        your_pointer: usize,
    }

}
