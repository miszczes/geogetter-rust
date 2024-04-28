pub mod url_encoder {
    use std::collections::HashMap;
    use std::fs;
    use std::io;
    use std::io::BufRead;
    use std::str;

    /// Generates a lookup map for URL encoding based on the contents of a file.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A string slice representing the path to the file containing encoding mappings.
    ///
    /// # Returns
    ///
    /// A Result containing a HashMap with characters mapped to their corresponding URL-encoded values if successful,
    /// or an io::Error if an error occurs.
    pub fn generate_lookup(file_path: &str) -> Result<HashMap<String, String>, io::Error> {
        let mut lookup_map = HashMap::new();
        let file = fs::File::open(file_path)?;
        let file_iterator = io::BufReader::new(file).lines();

        for (i, char) in file_iterator.enumerate() {
            let mut char = char.unwrap();
            if char.contains("space") {
                char = char.replace("space", " ");
            }
            lookup_map.insert(char, format!("%{:X}", 32 + i));
        }

        Ok(lookup_map)
    }

    /// Encodes a string using URL encoding, optionally skipping alphanumeric characters.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice representing the input string to be encoded.
    /// * `skip_alphanumeric` - A boolean indicating whether to skip encoding alphanumeric characters.
    ///
    /// # Returns
    ///
    /// A string representing the URL-encoded version of the input string.
    pub fn encode_str(input: &str, skip_alphanumeric: bool) -> String {
        let map = generate_lookup("map.txt").unwrap();
        let mut output = Vec::new();
        for c in input.chars() {
            if skip_alphanumeric && c.is_alphanumeric() {
                output.push(c.to_string())
            } else {
                output.push(map.get(&c.to_string()).cloned().unwrap())
            }
        }
        let vec_str = output.join("");
        vec_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_generate_lookup() {
        // Create a temporary file with some test data
        let file_path = "test_lookup.txt";
        let mut file = File::create(file_path).unwrap();
        file.write(b"space\na\nb\nc\nd\n").unwrap();
        // Generate lookup map from the temporary file
        let expected_lookup: HashMap<String, String> = [
            (" ".to_string(), "%20".to_string()), // "space" replaced by " "
            ("a".to_string(), "%21".to_string()),
            ("b".to_string(), "%22".to_string()),
            ("c".to_string(), "%23".to_string()),
            ("d".to_string(), "%24".to_string()),
        ]
        .iter()
        .cloned()
        .collect();

        // Call the function to generate the lookup map
        let result = url_encoder::generate_lookup(file_path);

        // Check if the function result is Ok
        assert!(result.is_ok());

        // Unwrap the result to get the actual lookup map
        let lookup_map = result.unwrap();

        // Compare the actual lookup map with the expected one
        assert_eq!(lookup_map, expected_lookup);

        // Clean up the temporary file
        std::fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_encode_string() {
        let input = " abcd";
        let expected_output = String::from("%20%61%62%63%64");

        assert_eq!(url_encoder::encode_str(input, false), expected_output);
    }

    #[test]
    fn test_encode_str_skip_alphanumeric() {
        assert_eq!(
            url_encoder::encode_str("Lebork, Polska", true),
            "Lebork%2C%20Polska"
        )
    }
}
