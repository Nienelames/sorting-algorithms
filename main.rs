use csv;
use rand::Rng;
use std::error::Error;
use plotters::prelude::*;
use std::fs::File;

struct SearchResult {
    target_index: i32,
    comparison_count: u32,
    array_length: usize,
}

struct SearchResults {
    binary_search_results: Vec<SearchResult>,
    interp_search_results: Vec<SearchResult>,
    interp_binary_search_results: Vec<SearchResult>,
}

fn main() {
    let num_arrays: Vec<Vec<u32>> = get_num_arrays();
    let mut num_generator = rand::thread_rng();
    let mut search_results = SearchResults{binary_search_results: Vec::with_capacity(1000), interp_search_results: Vec::with_capacity(1000), interp_binary_search_results: Vec::with_capacity(1000)};

    for num_array in num_arrays {
        let target_num: u32 = num_array[(num_generator.gen_range(0..num_array.len() - 1)) as usize];

        search_results.binary_search_results.push(binary_search(&num_array, target_num));
        search_results.interp_search_results.push(interpolation_search(&num_array, target_num));
        search_results.interp_binary_search_results.push(interpolated_binary_search(&num_array, target_num));
    }

    match write_search_results(&search_results) {
        Ok(_) => println!("Search results successfully written."),
        Err(e) => println!("Failed to write search results:\n{}", e),
    }

    match draw_result_graph(&search_results) {
        Ok(_) => println!("Graph successfully drawn"),
        Err(e) => println!("Failed to draw search result graph:\n{}", e),
    }
}

fn draw_result_graph(search_results: &SearchResults) -> Result<(), Box<dyn Error>> { 
    let root = SVGBackend::new("search_results.svg", (1920, 1000)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Search algorithm compplexity", ("sans-serif", (5).percent_height()))
        .set_label_area_size(LabelAreaPosition::Left, (8).percent())
        .set_label_area_size(LabelAreaPosition::Bottom, (4).percent())
        .margin((1).percent())
        .build_cartesian_2d(
            (20u32..50_0u32)
                .log_scale()
                .with_key_points(vec![10, 50, 100, 150, 200, 250, 400, 500]),
            (0u32..50_000u32)
                .log_scale()
                .with_key_points(vec![5, 10, 15, 20, 25, 30, 35]),
        )?;

    chart
        .configure_mesh()
        .x_desc("Array length")
        .y_desc("Comparison count")
        .draw()?;

    //Binary search line
    let mut color = Palette99::pick(0).mix(0.9);
    chart
        .draw_series(LineSeries::new(
            search_results.binary_search_results.iter().map(
                |&SearchResult {
                        array_length,
                        comparison_count,
                        ..
                    }| (array_length as u32, comparison_count as u32),
            ),
            color.stroke_width(3),
        ))?
        .label("Binary search")
        .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], color.filled()));

    //Interpolated search line
    color = Palette99::pick(1).mix(0.9);
    chart
        .draw_series(LineSeries::new(
            search_results.interp_search_results.iter().map(
                |&SearchResult {
                        array_length,
                        comparison_count,
                        ..
                    }| (array_length as u32, comparison_count as u32),
            ),
            color.stroke_width(3),
        ))?
        .label("Interpolation search")
        .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], color.filled()));

    //Interpolated binary search line
    color = Palette99::pick(2).mix(0.9);
    chart
        .draw_series(LineSeries::new(
            search_results.interp_binary_search_results.iter().map(
                |&SearchResult {
                        array_length,
                        comparison_count,
                        ..
                    }| (array_length as u32, comparison_count as u32),
            ),
            color.stroke_width(3),
        ))?
        .label("Interpolated binary search")
        .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], color.filled()));

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .draw()?;

    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", "search_results.svg");    
    
    Ok(())
}

fn write_search_results(search_results: &SearchResults) -> Result<(), Box<dyn Error>> {
    let mut search_res_writer = csv::Writer::from_path("search_results.csv")?;

    search_res_writer.write_record(&["Binary", "", "Interpolated", "", "Interpolated binary", ""])?;
    search_res_writer.write_record(&["Array length", "Comparison count", "Array length", "Comparison count", "Array length", "Comparison count"])?;

    for i in 0..1000 {
        search_res_writer.write_record(&[search_results.binary_search_results[i].array_length.to_string(), search_results.binary_search_results[i].comparison_count.to_string(),
                                        search_results.interp_search_results[i].array_length.to_string(), search_results.interp_search_results[i].comparison_count.to_string(),
                                        search_results.interp_binary_search_results[i].array_length.to_string(), search_results.interp_binary_search_results[i].comparison_count.to_string()])?;
    }

    Ok(())
}

fn get_num_arrays() -> Vec<Vec<u32>> {
    let mut num_generator = rand::thread_rng();
    let mut num_arrays: Vec<Vec<u32>> = Vec::new();
    
    for _ in 0..1000 {
        let array_size: u32 = num_generator.gen_range(2..500);
        let mut num_array: Vec<u32> = Vec::with_capacity(array_size as usize);
        
        let mut num_range: u32 = 0;
        for _ in 0..array_size {
            num_array.push(num_generator.gen_range(num_range..num_range + 10));
            num_range += 10;
        }

        num_arrays.push(num_array);
    }
    
    num_arrays.sort_unstable_by_key(Vec::len);

    num_arrays
}

fn binary_search(num_array: &Vec<u32>, target_num: u32) -> SearchResult {
    if num_array.len() == 0 {
        return SearchResult{target_index: -1, comparison_count: 1, array_length: num_array.len()}
    }

    let mut start_index: u32 = 0;
    let mut end_index: u32 = num_array.len() as u32 - 1;
    let mut search_index: u32;
    let mut comparison_count: u32 = 1;

    while start_index <= end_index {
        comparison_count += 1;
        search_index = start_index + (end_index - start_index) / 2;

        comparison_count += 1;
        if num_array[search_index as usize] == target_num {
            return SearchResult{array_length: num_array.len(), target_index: search_index as i32, comparison_count};
        }
        
        comparison_count += 1;
        if num_array[search_index as usize] < target_num {
            start_index = search_index + 1;
        } else { 
            end_index = search_index - 1;
        }
    }
    comparison_count += 1;

    SearchResult{array_length: num_array.len(), target_index: -1, comparison_count}
}

fn interpolation_search(num_array: &Vec<u32>, target_num: u32) -> SearchResult {
    if num_array.len() == 0 {
        return SearchResult{target_index: -1, comparison_count: 1, array_length: num_array.len()}
    }

    let mut start_index: u32 = 0;
    let mut end_index: u32 = num_array.len() as u32 - 1;
    let mut interp_index: u32;
    let mut comparison_count: u32 = 1;

    while num_array[end_index as usize] != num_array[start_index as usize] && target_num >= num_array[start_index as usize] && target_num <= num_array[end_index as usize] {
        comparison_count += 3;
        interp_index = start_index + (target_num - num_array[start_index as usize]) * (end_index - start_index) / (num_array[end_index as usize] - num_array[start_index as usize]);
 
        comparison_count += 1;
        if target_num == num_array[interp_index as usize] {
            return SearchResult{target_index: interp_index as i32, comparison_count, array_length: num_array.len()}
        }
        
        comparison_count += 1;
        if target_num < num_array[interp_index as usize] {
            end_index = interp_index - 1;
        }
        else {
            start_index = interp_index + 1;
        }
    }
 
    comparison_count += 4;
    if target_num == num_array[start_index as usize] {
        return SearchResult{target_index: start_index as i32, comparison_count, array_length: num_array.len()}
    }
    
    SearchResult{target_index: -1, comparison_count, array_length: num_array.len()}
}

fn interpolated_binary_search(num_array: &Vec<u32>, target_num: u32) -> SearchResult {
    if num_array.len() == 0 {
        return SearchResult{target_index: -1, comparison_count: 1, array_length: num_array.len()}
    }

    let mut start_index: u32 = 0;
    let mut end_index: u32 = num_array.len() as u32 - 1;
    let mut mid_index: u32;
    let mut inter_index: u32;
    let mut comparison_count: u32 = 1;

    while start_index < end_index {
        comparison_count += 1;
        inter_index = start_index + (target_num - num_array[start_index as usize]) * (end_index - start_index) / (num_array[end_index as usize] - num_array[start_index as usize]);
        
        comparison_count += 1;
        if target_num > num_array[inter_index as usize] {
            mid_index = (inter_index  + end_index) / 2;
        
            comparison_count += 1;
            if target_num <= num_array[mid_index as usize] {
                start_index = inter_index  + 1;
                end_index = mid_index; 
            } else {
                start_index = mid_index  + 1;
            }
        } else if target_num < num_array[inter_index as usize] {
            mid_index = (inter_index + start_index) / 2;
            
            comparison_count += 1;
            if target_num >= num_array[mid_index as usize] {
                start_index = mid_index;
                end_index = inter_index - 1;
            } else {
                end_index = mid_index - 1;
            }
        } else {
            return SearchResult{target_index: inter_index as i32, comparison_count, array_length: num_array.len()}
        }
        comparison_count += 1;
    } 
    
    comparison_count += 2;
    if target_num == num_array[start_index as usize] {
        return SearchResult{target_index: start_index as i32, comparison_count, array_length: num_array.len()}
    }
    
    return SearchResult{target_index: -1, comparison_count, array_length: num_array.len()}
}