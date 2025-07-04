use std::collections::HashMap;

pub fn pack_domain(domain: &str, labels_map: &mut HashMap<String, usize>, off: usize, compress: bool) -> Vec<u8> {
    if domain.is_empty() {
        return vec![0x00];
    }

    let mut buf = Vec::new();
    let mut off = off;

    let parts: Vec<&str> = domain.split('.').collect();

    for i in 0..parts.len() {
        let label = parts[i..].join(".");

        if compress {
            if let Some(&ptr_offset) = labels_map.get(&label) {
                buf.extend_from_slice(&[
                    (0xC0 | ((ptr_offset >> 8) & 0x3F)) as u8,
                    (ptr_offset & 0xFF) as u8
                ]);
                return buf;
            }
        }

        let label_bytes = parts[i].as_bytes();
        buf.push(label_bytes.len() as u8);
        buf.extend_from_slice(label_bytes);
        labels_map.insert(label, off);
        off += label_bytes.len() + 1;
    }

    buf.push(0x00);

    buf
}

pub fn unpack_domain(buf: &[u8], off: usize) -> (String, usize) {
    let mut builder = String::new();
    let mut pos = off;
    let mut jumped = false;
    let mut original_pos = pos;

    while pos < buf.len() {
        let length = buf[pos] as usize;
        pos += 1;

        if length == 0 {
            break;
        }

        if (length & 0xC0) == 0xC0 {
            if pos >= buf.len() {
                break;
            }
            let pointer_offset = ((length & 0x3F) << 8) | buf[pos] as usize;
            pos += 1;

            if !jumped {
                original_pos = pos;
            }
            pos = pointer_offset;
            jumped = true;

        } else {
            if !builder.is_empty() {
                builder.push('.');
            }

            if pos + length > buf.len() {
                break;
            }

            let label = &buf[pos..pos + length];
            builder.push_str(&String::from_utf8_lossy(label));
            pos += length;
        }
    }

    let final_pos = if jumped { original_pos } else { pos };
    (builder.to_lowercase(), final_pos - off)
}
