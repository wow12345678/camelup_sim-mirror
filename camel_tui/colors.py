from PIL import Image

def find_non_white_pixels(image_path):
    """
    Create a boolean array where True = non-white, False = white.
    
    Args:
        image_path: Path to the image file
        
    Returns:
        List of lists where each bool represents if that pixel is non-white
    """
    # Open the image
    img = Image.open(image_path)
    
    # Convert to RGB if necessary
    if img.mode != 'RGB':
        img = img.convert('RGB')
    
    # Get image dimensions
    width, height = img.size
    
    # Initialize result list
    result = []
    
    # Iterate over all pixels
    for y in range(height):
        row = []
        for x in range(width):
            # Get pixel RGB values
            r, g, b = img.getpixel((x, y))
            
            # True if not white, False if white
            is_non_white = not (r == 255 and g == 255 and b == 255)
            row.append(is_non_white)
        
        result.append(row)
    
    return result


def generate_rust_constant(bool_array):
    """Generate Rust constant array from boolean array."""
    print("const NON_WHITE_PIXELS: &[&[bool]] = &[")
    
    for row in bool_array:
        bool_str = ', '.join('true' if b else 'false' for b in row)
        print(f"    &[{bool_str}],")
    
    print("];")


# Example usage
if __name__ == "__main__":
    image_path = "camel1.png"  # Replace with your image path
    
    non_white_bool = find_non_white_pixels(image_path)
    
    # Print image dimensions
    height = len(non_white_bool)
    width = len(non_white_bool[0]) if non_white_bool else 0
    print(f"Image dimensions: {width}x{height}")
    
    # Count total non-white pixels
    total = sum(sum(row) for row in non_white_bool)
    print(f"Total non-white pixels: {total}")
    
    # Generate Rust constant
    print("\n" + "="*50)
    print("Rust constant (copy this into your Rust code):")
    print("="*50)
    generate_rust_constant(non_white_bool)
