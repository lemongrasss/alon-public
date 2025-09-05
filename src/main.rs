use quicksort::quicksort;

fn main() {
    // Example usage of quicksort
    let mut numbers = [64, 34, 25, 12, 22, 11, 90];
    
    println!("Original array: {:?}", numbers);
    
    quicksort(&mut numbers);
    
    println!("Sorted array: {:?}", numbers);
    
    // Another example with strings
    let mut words = ["banana", "apple", "cherry", "date"];
    println!("\nOriginal words: {:?}", words);
    
    quicksort(&mut words);
    println!("Sorted words: {:?}", words);
}