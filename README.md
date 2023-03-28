# 🐟 F1sh

CLI tool to convert image files.

## 🖼️ Supported formats

|   File   | Supported |
| :------: | :-------: |
| AviF     |    ✅     |
| BMP      |    ✅     |
| DDS      |    ✅     |
| Farbfeld |    ✅     |
| GIF      |    ✅     |
| HEIF     |    ❌     |
| ICO      |    ✅     |
| JPEG     |    ✅     |
| OpenEXR  |    ✅     |
| PNG      |    ✅     |
| PNM      |    ✅     |
| RAW      |    ❌     |
| SVG      |    ❌     |
| TIFF     |    ✅     |
| TGA      |    ✅     |
| WebP     |    ✅     |

## ⚙️ Get started

### Convert a image

Convert the enteder image file.

```bash
# Command
f1sh convert [...options]

# Examples:
#  - Save under same name but different extension
f1sh convert -i /home/sammwy/image.webp -f jpg
#  - Save under specific output  file.
f1sh convert -i /home/sammwy/image.webp -f jpg -o /home/other/image.jpg
```

#### Convert Options

|     Name    | Short key |           Description          | Default Value |
|:------------|:---------:|:------------------------------:| ------------: |
| --format    |    -f     |   Format/extension of output   |      ❌      |
| --input     |    -i     |     Path of file to convert    |      ❌      |
| --output    |    -o     |       Path of output file      | Same of input but diff extension |

## 🤝 Contributing

Contributions, issues and feature requests are welcome!
Feel free to check [issues page](https://github.com/sammwyy/F1sh/issues).

## ❤️ Show your support

Give a ⭐️ if this project helped you!

Or buy me a coffeelatte 🙌

[Ko-fi](https://ko-fi.com/sammwy) | [Patreon](https://patreon.com/sammwy)

## 📝 License

Copyright © 2023 [Sammwy](https://github.com/sammwyy).  
This project is [MIT](LICENSE) licensed.
