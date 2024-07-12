use base64::prelude::*;
use pyo3::prelude::*;
use pyo3::types::{PyInt, PyString};
use thumbhash::{rgba_to_thumb_hash, thumb_hash_to_rgba};

#[pyfunction]
fn encode_image_to_thumbhash(
    base64_string: &Bound<'_, PyString>,
    width: &Bound<'_, PyInt>,
    height: &Bound<'_, PyInt>,
) -> PyResult<Vec<u8>> {
    let width_usize = width.extract::<usize>()?;
    let height_usize = height.extract::<usize>()?;
    let base64_string = base64_string.to_str()?;

    let image_data = BASE64_STANDARD.decode(base64_string).unwrap();
    let thumbhash = rgba_to_thumb_hash(width_usize, height_usize, &image_data);
    Ok(thumbhash.to_vec())
}

#[pyfunction]
fn decode_thumbhash_to_image(hash: &Bound<'_, PyString>) -> PyResult<Vec<u8>> {
    let hash_string = hash.to_str()?;
    let hash = hash_string.as_bytes().to_vec();
    let (_w, _h, rgba2) = thumb_hash_to_rgba(&hash).unwrap();
    Ok(rgba2.to_vec())
}

#[pymodule]
fn thumbhash_pyrust(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode_image_to_thumbhash, m)?)?;
    m.add_function(wrap_pyfunction!(decode_thumbhash_to_image, m)?)?;
    Ok(())
}
