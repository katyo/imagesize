#[cfg(test)]
use ::*;

#[test]
fn bmp_test() {
    let dim = size("test/test.bmp").unwrap();
    assert_eq!(dim.width, 512);
    assert_eq!(dim.height, 512);
}

#[test]
fn gif_test() {
    let dim = size("test/test.gif").unwrap();
    assert_eq!(dim.width, 100);
    assert_eq!(dim.height, 100);
}

#[test]
fn jpeg_test() {
    let dim = size("test/test.jpg").unwrap();
    assert_eq!(dim.width, 690);
    assert_eq!(dim.height, 298);
}

#[test]
fn jpeg_extra_info_test() {
    let dim = size("test/extra.jpg").unwrap();
    assert_eq!(dim.width, 1500);
    assert_eq!(dim.height, 844);
}

#[test]
fn png_test() {
    let dim = size("test/test.png").unwrap();
    assert_eq!(dim.width, 2000);
    assert_eq!(dim.height, 2000);
}

#[test]
fn webp_test() {
    let dim = size("test/test.webp").unwrap();
    assert_eq!(dim.width, 716);
    assert_eq!(dim.height, 716);
}

#[test]
fn riffx_webp_test() {
    let dim = size("test/riffx.webp").unwrap();
    assert_eq!(dim.width, 128);
    assert_eq!(dim.height, 128);
}

#[test]
fn blob_test() {
    //  PNG Header with size 123x321
    let data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 
                    0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, 
                    0x00, 0x00, 0x00, 0x7B, 0x00, 0x00, 0x01, 0x41,
                    0x08, 0x06, 0x00, 0x00, 0x00, 0x9A, 0x38, 0xC4];

    let dim = blob_size(&data).unwrap();
    assert_eq!(dim.width, 123);
    assert_eq!(dim.height, 321);
}

#[test]
fn blob_too_small_test() {
    let data = vec![0x89, 0x00, 0x01, 0x02];
    assert_eq!(blob_size(&data).is_err(), true);
}

#[test]
fn blob_test_fail() {
    //  Invalid PNG header (0x51 instead of 0x50)
    let data = vec![0x89, 0x51, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 
                    0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, 
                    0x00, 0x00, 0x00, 0x7B, 0x00, 0x00, 0x01, 0x41,
                    0x08, 0x06, 0x00, 0x00, 0x00, 0x9A, 0x38, 0xC4];

    assert_eq!(blob_size(&data).is_err(), true);
}

#[test]
fn gif_blob_too_small_test() {
    let data = vec![0x47, 0x49, 0x46, 0x38];
    assert_eq!(blob_size(&data).is_err(), true);
}
