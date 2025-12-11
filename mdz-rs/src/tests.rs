#[cfg(test)]
mod tests {
    use crate::{is_url, get_asset_type, get_asset_subdir, extract_images_from_markdown, pack, unpack, copy_local_file, Asset, Manifest};
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    // 创建临时测试文件
    fn create_test_file(dir: &Path, name: &str, content: &str) -> std::path::PathBuf {
        let file_path = dir.join(name);
        fs::write(&file_path, content).unwrap();
        file_path
    }

    // 创建临时测试图片
    fn create_test_image(dir: &Path, name: &str) -> std::path::PathBuf {
        let file_path = dir.join(name);
        let svg_content = "<svg width=\"100\" height=\"100\" xmlns=\"http://www.w3.org/2000/svg\">
  <rect width=\"100\" height=\"100\" fill=\"#f0f0f0\"/>
  <text x=\"50\" y=\"50\" font-family=\"Arial\" font-size=\"14\" text-anchor=\"middle\" fill=\"#333\">Test</text>
</svg>";
        fs::write(&file_path, svg_content).unwrap();
        file_path
    }

    #[test]
    fn test_is_url() {
        // 测试有效的 URL
        assert!(is_url("https://example.com/image.png"));
        assert!(is_url("http://test.com/photo.jpg"));
        assert!(is_url("https://sub.domain.org/path/to/image.svg"));

        // 测试无效的 URL（本地路径）
        assert!(!is_url("./local/image.png"));
        assert!(!is_url("/absolute/path/image.jpg"));
        assert!(!is_url("../relative/path/image.gif"));
        assert!(!is_url("image.png"));

        // 测试边缘情况
        assert!(!is_url(""));
        assert!(!is_url("not-a-url"));
        assert!(!is_url("://missing-protocol.com"));
    }

    #[test]
    fn test_get_asset_type() {
        // 测试图片类型
        assert_eq!(get_asset_type(Path::new("image.png")), "image");
        assert_eq!(get_asset_type(Path::new("photo.jpg")), "image");
        assert_eq!(get_asset_type(Path::new("graphic.jpeg")), "image");
        assert_eq!(get_asset_type(Path::new("icon.gif")), "image");
        assert_eq!(get_asset_type(Path::new("logo.svg")), "image");
        assert_eq!(get_asset_type(Path::new("bitmap.bmp")), "image");
        assert_eq!(get_asset_type(Path::new("webp.webp")), "image");
        assert_eq!(get_asset_type(Path::new("favicon.ico")), "image");

        // 测试视频类型
        assert_eq!(get_asset_type(Path::new("video.mp4")), "video");
        assert_eq!(get_asset_type(Path::new("movie.avi")), "video");
        assert_eq!(get_asset_type(Path::new("clip.mov")), "video");
        assert_eq!(get_asset_type(Path::new("recording.wmv")), "video");
        assert_eq!(get_asset_type(Path::new("stream.webm")), "video");
        assert_eq!(get_asset_type(Path::new("movie.mkv")), "video");
        assert_eq!(get_asset_type(Path::new("video.flv")), "video");

        // 测试音频类型
        assert_eq!(get_asset_type(Path::new("audio.mp3")), "audio");
        assert_eq!(get_asset_type(Path::new("sound.wav")), "audio");
        assert_eq!(get_asset_type(Path::new("music.ogg")), "audio");
        assert_eq!(get_asset_type(Path::new("track.flac")), "audio");
        assert_eq!(get_asset_type(Path::new("song.aac")), "audio");

        // 测试其他文件类型
        assert_eq!(get_asset_type(Path::new("document.pdf")), "file");
        assert_eq!(get_asset_type(Path::new("text.txt")), "file");
        assert_eq!(get_asset_type(Path::new("data.csv")), "file");
        assert_eq!(get_asset_type(Path::new("presentation.pptx")), "file");

        // 测试无扩展名的情况
        assert_eq!(get_asset_type(Path::new("noextension")), "file");

        // 测试大小写不敏感
        assert_eq!(get_asset_type(Path::new("image.PNG")), "image");
        assert_eq!(get_asset_type(Path::new("video.MP4")), "video");
        assert_eq!(get_asset_type(Path::new("audio.MP3")), "audio");
    }

    #[test]
    fn test_get_asset_subdir() {
        assert_eq!(get_asset_subdir("image"), "images");
        assert_eq!(get_asset_subdir("video"), "videos");
        assert_eq!(get_asset_subdir("audio"), "audio");
        assert_eq!(get_asset_subdir("file"), "files");
        assert_eq!(get_asset_subdir("unknown"), "files");
    }

    #[test]
    fn test_extract_images_from_markdown() {
        // 测试标准 Markdown 图片语法
        let markdown1 = r#"Here is an image: ![alt text](./image.png)
And another one: ![Another image](../photos/image.jpg)"#;
        let images1 = extract_images_from_markdown(markdown1);
        assert_eq!(images1.len(), 2);
        assert_eq!(images1[0], ("./image.png".to_string(), Some("alt text".to_string())));
        assert_eq!(images1[1], ("../photos/image.jpg".to_string(), Some("Another image".to_string())));

        // 测试 HTML img 标签
        let markdown2 = r#"<img src="./image.png" alt="HTML Image">
<img src="photo.jpg">"#;
        let images2 = extract_images_from_markdown(markdown2);
        assert_eq!(images2.len(), 2);
        assert_eq!(images2[0], ("./image.png".to_string(), Some("HTML Image".to_string())));
        assert_eq!(images2[1], ("photo.jpg".to_string(), None));

        // 测试混合内容
        let markdown3 = r#"![Markdown Image](./img1.png)
Some text here.
<img src="./img2.jpg" alt="HTML Image">
More text.
![Another MD Image](img3.svg)"#;
        let images3 = extract_images_from_markdown(markdown3);
        assert_eq!(images3.len(), 3);
        // 按照函数的执行顺序：先 Markdown 格式，再 HTML 格式
        assert_eq!(images3[0], ("./img1.png".to_string(), Some("Markdown Image".to_string())));
        assert_eq!(images3[1], ("img3.svg".to_string(), Some("Another MD Image".to_string())));
        assert_eq!(images3[2], ("./img2.jpg".to_string(), Some("HTML Image".to_string())));

        // 测试无图片的情况
        let markdown4 = "This is just text without any images.";
        let images4 = extract_images_from_markdown(markdown4);
        assert_eq!(images4.len(), 0);

        // 测试 assets:// URL 应该被跳过
        let markdown5 = "![Local Asset](assets://img1.png)";
        let images5 = extract_images_from_markdown(markdown5);
        assert_eq!(images5.len(), 1); // 仍然会提取，但在 pack 时会被跳过
        assert_eq!(images5[0], ("assets://img1.png".to_string(), Some("Local Asset".to_string())));
    }

    #[tokio::test]
    async fn test_pack_with_local_images() {
        // 创建临时目录
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // 创建测试 Markdown 文件
        let markdown_content = r#"# Test Document

This is a test document with local images.

![Local Image 1](./image1.png)
![Local Image 2](./graphic.svg)

<img src="./image3.jpg" alt="HTML Image">
"#;
        let md_file = create_test_file(temp_path, "test.md", markdown_content);

        // 创建测试图片文件
        create_test_image(temp_path, "image1.png");
        create_test_image(temp_path, "graphic.svg");
        create_test_image(temp_path, "image3.jpg");

        // 打包
        let output_file = temp_path.join("test.mdz").to_str().unwrap().to_string();
        let md_file_str = md_file.to_str().unwrap();

        let result = pack(md_file_str, &output_file, Some("Test Document".to_string()), Some("Test Author".to_string())).await;
        assert!(result.is_ok(), "Pack failed: {:?}", result.err());

        // 验证输出文件存在
        assert!(Path::new(&output_file).exists());

        // 解包验证 - 切换到临时目录进行解包
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&temp_path).unwrap();

        let unpack_result = unpack(&output_file, None);
        assert!(unpack_result.is_ok(), "Unpack failed: {:?}", unpack_result.err());

        // 由于解包到当前目录，检查当前目录
        assert!(temp_path.join("test.md").exists());
        assert!(temp_path.join("assets/images/image1.png").exists());
        assert!(temp_path.join("assets/images/graphic.svg").exists());
        assert!(temp_path.join("assets/images/image3.jpg").exists());

        // 恢复原始目录
        std::env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_unpack_invalid_file() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // 创建无效的 ZIP 文件
        let invalid_file = temp_path.join("invalid.mdz");
        fs::write(&invalid_file, "This is not a valid ZIP file").unwrap();

        let result = unpack(invalid_file.to_str().unwrap(), None);
        assert!(result.is_err());
    }

    #[test]
    fn test_pack_nonexistent_file() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let output_file = temp_path.join("output.mdz").to_str().unwrap().to_string();
        let nonexistent_file = temp_path.join("nonexistent.md").to_str().unwrap().to_string();

        // 由于 pack 是 async，我们需要在 runtime 中运行
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(async {
            pack(&nonexistent_file, &output_file, None, None).await
        });

        assert!(result.is_err());
    }

    #[test]
    fn test_copy_local_file() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // 创建源文件
        let source_file = temp_path.join("source.txt");
        let source_content = "Hello, World!";
        fs::write(&source_file, source_content).unwrap();

        // 创建目标路径
        let dest_file = temp_path.join("subdir/dest.txt");

        // 复制文件
        let result = copy_local_file(&source_file, &dest_file);
        assert!(result.is_ok());

        // 验证文件被复制
        assert!(dest_file.exists());
        let dest_content = fs::read_to_string(&dest_file).unwrap();
        assert_eq!(dest_content, source_content);
    }

    #[test]
    fn test_manifest_serialization() {
        let manifest = Manifest {
            version: "1.0.0".to_string(),
            title: "Test Document".to_string(),
            author: Some("Test Author".to_string()),
            date: Some("2025-12-11".to_string()),
            filename: "test.md".to_string(),
            assets: vec![
                Asset {
                    id: "image1".to_string(),
                    path: "assets/images/image1.png".to_string(),
                    asset_type: "image".to_string(),
                    alt: Some("Test Image".to_string()),
                    title: Some("Image Title".to_string()),
                },
                Asset {
                    id: "video1".to_string(),
                    path: "assets/videos/video1.mp4".to_string(),
                    asset_type: "video".to_string(),
                    alt: None,
                    title: None,
                },
            ],
        };

        // 测试序列化
        let json = serde_json::to_string_pretty(&manifest).unwrap();

        // 测试反序列化
        let deserialized: Manifest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.version, manifest.version);
        assert_eq!(deserialized.title, manifest.title);
        assert_eq!(deserialized.author, manifest.author);
        assert_eq!(deserialized.date, manifest.date);
        assert_eq!(deserialized.filename, manifest.filename);
        assert_eq!(deserialized.assets.len(), manifest.assets.len());
        assert_eq!(deserialized.assets[0].id, manifest.assets[0].id);
        assert_eq!(deserialized.assets[1].id, manifest.assets[1].id);
    }
}