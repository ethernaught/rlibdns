pub mod messages;
pub mod records;
pub mod utils;
pub mod zone;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::messages::inter::rr_types::RRTypes;
    use crate::messages::message_base::MessageBase;
    use crate::records::inter::record_base::RecordBase;
    use crate::zone::zone_parser::ZoneParser;

    type RecordMap = HashMap<String, HashMap<RRTypes, Vec<Box<dyn RecordBase>>>>;

    #[test]
    fn encode_and_decode() {
        let x = vec![ 0xa7, 0xa2, 0x81, 0x80, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x06, 0x67, 0x6f, 0x6f,
                      0x67, 0x6c, 0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01, 0x00, 0x01, 0xc0, 0x0c, 0x00, 0x01,
                      0x00, 0x01, 0x00, 0x00, 0x01, 0x23, 0x00, 0x04, 0x8e, 0xfa, 0x45, 0xee, 0x00, 0x00, 0x29, 0x04,
                      0xd0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 ];

        //URI
        let x = vec![ 0x8e, 0x39, 0x85, 0x0, 0x0, 0x1, 0x0, 0x1, 0x0, 0x0, 0x0, 0x1, 0x5, 0x66, 0x69, 0x6e, 0x64, 0x39, 0x3, 0x6e, 0x65, 0x74, 0x0, 0x1, 0x0, 0x0, 0x1, 0xc0, 0xc, 0x1, 0x0, 0x0, 0x1, 0x0, 0x0, 0x1, 0x2c, 0x0, 0x16, 0x0, 0x1, 0x0, 0x1, 0x66, 0x69, 0x6e, 0x64, 0x39, 0x3a, 0x2f, 0x2f, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x0, 0x0, 0x29, 0x4, 0xd0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0 ];

        //LOC
        let x = vec![ 0xb6, 0x23, 0x85, 0x0, 0x0, 0x1, 0x0, 0x1, 0x0, 0x0, 0x0, 0x1, 0x5, 0x66, 0x69, 0x6e, 0x64, 0x39, 0x3, 0x6e, 0x65, 0x74, 0x0, 0x0, 0x1d, 0x0, 0x1, 0xc0, 0xc, 0x0, 0x1d, 0x0, 0x1, 0x0, 0x0, 0x1, 0x2c, 0x0, 0x10, 0x0, 0x0, 0x0, 0x0, 0x6e, 0x67, 0x2d, 0xa0, 0x9c, 0xf7, 0xc5, 0x80, 0x0, 0x98, 0x96, 0x80, 0x0, 0x0, 0x29, 0x4, 0xd0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0 ];

        //ANY
        let x = vec![ 0x9c, 0x84, 0x85, 0x0, 0x0, 0x1, 0x0, 0x1, 0x0, 0x0, 0x0, 0x1, 0x5, 0x66, 0x69, 0x6e, 0x64, 0x39, 0x3, 0x6e, 0x65, 0x74, 0x0, 0x0, 0xff, 0x0, 0x1, 0xc0, 0xc, 0x0, 0xd, 0x0, 0x1, 0x0, 0x0, 0xe, 0x10, 0x0, 0x9, 0x7, 0x52, 0x46, 0x43, 0x38, 0x34, 0x38, 0x32, 0x0, 0x0, 0x0, 0x29, 0xff, 0xff, 0x0, 0x0, 0x0, 0x0, 0x0, 0x32, 0x0, 0xf, 0x0, 0x2e, 0x0, 0x15, 0x54, 0x79, 0x70, 0x65, 0x20, 0x41, 0x4e, 0x59, 0x20, 0x51, 0x75, 0x65, 0x72, 0x69, 0x65, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x20, 0x68, 0x65, 0x72, 0x65, 0x2c, 0x20, 0x52, 0x46, 0x43, 0x38, 0x34, 0x38, 0x32 ];

        let x = vec![ 0x45, 0xe, 0x85, 0x0, 0x0, 0x1, 0x0, 0x1, 0x0, 0x0, 0x0, 0x1, 0x5, 0x66, 0x69, 0x6e, 0x64, 0x39, 0x3, 0x6e, 0x65, 0x74, 0x0, 0x0, 0x2c, 0x0, 0x1, 0xc0, 0xc, 0x0, 0x2c, 0x0, 0x1, 0x0, 0x0, 0x1, 0x2c, 0x0, 0x16, 0x1, 0x2, 0x8b, 0x9f, 0x2f, 0x2b, 0x6b, 0x3b, 0x6, 0xe3, 0xd8, 0x9f, 0x54, 0xd0, 0x73, 0xd, 0xce, 0x5b, 0x4, 0x26, 0xf8, 0x8c, 0x0, 0x0, 0x29, 0x4, 0xd0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0 ];

        let message = MessageBase::from_bytes(&x).unwrap();
        
        println!("{}", message);

        assert_eq!(x, message.to_bytes(512));
    }

    #[test]
    fn parsing() {
        let mut records = RecordMap::new();

        let mut parser = ZoneParser::new("/home/brad/Downloads/find9.net.test.zone", "find9.net").unwrap();
        for (name, record) in parser.iter() {
            println!("{}: {}", name, record);

            records
                .entry(name)
                .or_insert_with(HashMap::new)
                .entry(record.get_type())
                .or_insert_with(Vec::new)
                .push(record);
        }

        //println!("{:?}", records);
        println!("{:?}", records["@"][&RRTypes::A]);
    }
}
