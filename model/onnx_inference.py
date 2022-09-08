import onnxruntime as ort

sess = ort.InferenceSession("/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/model/model.onnx")
