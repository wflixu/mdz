pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn pack(input_dir: &str, output_file: &str) -> anyhow::Result<()> {
    println!("Packing '{}' into '{}'", input_dir, output_file);
    // TODO: 实现 ZIP 打包逻辑
    Ok(())
}

pub fn unpack(input_file: &str, output_dir: &str) -> anyhow::Result<()> {
    println!("Unpacking '{}' into '{}'", input_file, output_dir);
    // TODO: 实现 ZIP 解压逻辑
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
