#[cfg(test)]
mod tests {

    #[test]
    fn can_parse_classfile() {
        classfile_parser::parse_class("../examples/fundamentals/Empty").unwrap();
    }
}
