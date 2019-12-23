pub fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|string| string.trim())
        .filter(|string| !string.is_empty())
        .map(|part| {
            part.chars()
                .map(|part| match part {
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    _ => panic!("unhandled char {:?}", part),
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>()
}
