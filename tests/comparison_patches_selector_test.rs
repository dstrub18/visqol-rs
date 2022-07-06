use visqol_rs::{comparison_patches_selector::ComparisonPatchesSelector, neurogram_similiarity_index_measure::NeurogramSimiliarityIndexMeasure, audio_signal::AudioSignal, image_patch_creator::ImagePatchCreator, patch_creator::PatchCreator};
use ndarray::{Array2, arr2};
#[test]
fn test_calc_num_patches()
{
    let patch_indices = vec![0, 15, 30, 45, 60];

    let slide_offset = 45;
    let accepted_num_patches = ComparisonPatchesSelector::calc_max_num_patches(&patch_indices, slide_offset, 30);

    assert_eq!(patch_indices.len(), accepted_num_patches);
    
    let slide_offset = 44;
    let accepted_num_patches = ComparisonPatchesSelector::calc_max_num_patches(&patch_indices, slide_offset, 30);

    assert_eq!(patch_indices.len() - 1, accepted_num_patches);
}

#[test]
fn test_slice()
{
    let fs = 16000;
    let num_seconds = 3;

    let mut silence_matrix = Array2::zeros((fs * num_seconds, 1));

    silence_matrix[(16000, 0)] = 1.0;
    let three_seconds_silence = AudioSignal::new(silence_matrix, fs as u32);

    let sliced_signal = ComparisonPatchesSelector::slice(&three_seconds_silence, 0.5, 2.5);

    assert_eq!(sliced_signal.get_duration(), 2.0);
    assert_eq!(sliced_signal.data_matrix[(7999, 0)], 0.0);
    assert_eq!(sliced_signal.data_matrix[(8000, 0)], 1.0);
    assert_eq!(sliced_signal.data_matrix[(8001, 0)], 0.0);
}

#[test]
fn test_optimal_patches()
{
    let ref_matrix = arr2(&
        [
        [1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0, 2.0, 3.0, 3.0],
        [0.0, 1.0, 0.0, 2.0, 1.0, 1.0, 2.0, 3.0, 1.0, 2.0],
        [0.0, 1.0, 0.0, 2.0, 1.0, 1.0, 2.0, 3.0, 1.0, 2.0]
         ]);
    
    // Create reference patches from given patch indicesc
    let patch_size = 1;
    let mut patch_indices: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let patch_creator = ImagePatchCreator::new(patch_size);
    let mut ref_patches = patch_creator.create_patches_from_indices(&ref_matrix, &patch_indices);
    
    // Defining the degraded audio matrix
    let rows_concatenated: Vec<f64> = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0, 0.0, 0.0, 0.0, 0.0, 2.0, 2.0, 3.0, 2.0, 0.0, 0.0, 0.0, 0.0, 3.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    assert_eq!(rows_concatenated.len(), 3*30);
    let mut deg_matrix = Array2::from_shape_vec((3,30), rows_concatenated).unwrap();

    let frame_duration = 1.0;
    let search_window = 8;

    let sim_measurer = NeurogramSimiliarityIndexMeasure::default();
    let selector = ComparisonPatchesSelector::new(sim_measurer);

    let res = selector.find_most_optimal_deg_patches(&mut ref_patches, &mut patch_indices, &mut deg_matrix, frame_duration, search_window);
    
    assert_eq!(res[3].deg_patch_start_time, 0.0);
    assert_eq!(res[4].deg_patch_start_time, 7.0);
    assert_eq!(res[5].deg_patch_start_time, 8.0);

}

#[test]
fn out_of_order_matches()
{
    let ref_matrix = arr2(
        &[
            [1.0, 100.0, 3.0, 4.0],
            [0.0;4],
            [1.0, 100.0, 3.0, 4.0]]);

    let patch_size = 1;
    let mut patch_indices = vec![0,1,2,3];

    let patch_creator = ImagePatchCreator::new(patch_size);
    let mut ref_patches = patch_creator.create_patches_from_indices(&ref_matrix, &patch_indices);
    
    let mut deg_matrix = arr2(
        &[
            [100.0, 1.0, 3.0, 4.0],
            [0.0;4],
            [100.0, 1.0, 3.0, 4.0]]);

    let frame_duration = 1.0;
    let search_window = 60;

    
    let sim_measurer = NeurogramSimiliarityIndexMeasure::default();
    let selector = ComparisonPatchesSelector::new(sim_measurer);

    let res = selector.find_most_optimal_deg_patches(&mut ref_patches, &mut patch_indices, &mut deg_matrix, frame_duration, search_window);

    assert_eq!(res[0].deg_patch_start_time, 1.0);
    assert_eq!(res[1].deg_patch_start_time, 0.0);
    assert_eq!(res[2].deg_patch_start_time, 2.0);
    assert_eq!(res[3].deg_patch_start_time, 3.0);
}

#[test]
fn different_results()
{
    let ref_matrix = arr2(&[[1.0], [1.0], [0.0]]);

    let patch_size = 1;
    let mut patch_indices = vec![0];

    let patch_creator = ImagePatchCreator::new(patch_size);
    let mut ref_patches = patch_creator.create_patches_from_indices(&ref_matrix, &patch_indices);
    

    let concatenated_deg_mat = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let mut deg_matrix = Array2::from_shape_vec((3,17), concatenated_deg_mat).unwrap();

    let frame_duration = 1.0;
    let search_window = 60;

    
    let sim_measurer = NeurogramSimiliarityIndexMeasure::default();
    let selector = ComparisonPatchesSelector::new(sim_measurer);

    let res = selector.find_most_optimal_deg_patches(&mut ref_patches, &mut patch_indices, &mut deg_matrix, frame_duration, search_window);
    assert_eq!(res[0].deg_patch_start_time, 6.0);
}

#[test]
fn big_example()
{
    let ref_vec = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 2.0, 2.0, 2.0, 2.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                                 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                                 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

    let ref_matrix = Array2::from_shape_vec((3,31), ref_vec).unwrap();

    let patch_size = 2;

    let mut patch_indices = vec![4, 6, 10, 12, 14, 22];

    let patch_creator = ImagePatchCreator::new(patch_size);
    let mut ref_patches = patch_creator.create_patches_from_indices(&ref_matrix, &patch_indices);
    
    let deg_vec = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0, 0.0, 0.0, 0.0, 0.0, 2.0, 1.0, 2.0, 3.0, 2.0, 0.0, 0.0, 0.0, 0.0, 3.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                                 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                                 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

    let mut deg_matrix = Array2::from_shape_vec((3,31), deg_vec).unwrap();

    let frame_duration = 1.0;
    let search_window = 60;

    
    let sim_measurer = NeurogramSimiliarityIndexMeasure::default();
    let selector = ComparisonPatchesSelector::new(sim_measurer);

    let res = selector.find_most_optimal_deg_patches(&mut ref_patches, &mut patch_indices, &mut deg_matrix, frame_duration, search_window);

    assert_eq!(res[0].deg_patch_start_time, 6.0);
    assert_eq!(res[1].deg_patch_start_time, 8.0);
    assert_eq!(res[2].deg_patch_start_time, 12.0);
    assert_eq!(res[3].deg_patch_start_time, 14.0);
    assert_eq!(res[4].deg_patch_start_time, 16.0);
    assert_eq!(res[5].deg_patch_start_time, 22.0);
}