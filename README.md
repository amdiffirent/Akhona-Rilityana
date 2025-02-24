# Linear Regression Model

##  Project Overview
This project implements a simple **Linear Regression Model** using Rust. The model generates random data points and applies linear regression to make predictions. It also visualizes the results using text-based plotting.

##  Setup and Installation

### **1️⃣ Prerequisites**
Before running the project, ensure you have the following installed:
- [Rust](https://www.rust-lang.org/tools/install) (latest version)
- [Cargo](https://doc.rust-lang.org/cargo/) (Rust package manager)
- A Rust-compatible IDE (e.g., **Visual Studio Code** with the Rust extension)

### **2️⃣ Clone the Repository**
```sh
git clone https://github.com/amdiffirent/Akhona_Rilityana/linear_regression_model.git
cd linear_regression_model
```

### **3️⃣ Build and Run the Project**
```sh
cargo build  # Compile the project
cargo run    # Run the project
```

##  How It Works
1. Generates random **x** values and corresponding **y** values with added noise.
2. Trains a **linear regression model**.
3. Makes predictions based on new data points.
4. Displays results using text-based plotting.

##  Code Structure
```bash
linear_regression_model/
│── src/
│   ├── main.rs        # Main Rust file
│── Cargo.toml         # Rust dependencies
│── README.md         # Project documentation
```

## Dependencies Used
The project uses the following Rust crates:
- **ndarray** → For numerical operations
- **rand** → To generate random numbers
- **textplots** → To visualize the data
- **linregress** → For performing linear regression

##  Challenges Faced
1. **Virtualization Issues**  
   - **Problem:** Initially used `NdArrayDevice`, leading to unexpected tensor behavior.  
   - **Solution:** Switched to the **NdArray backend** to ensure smooth tensor operations.

2. **Random Number Generation Warnings**  
   - **Problem:** `rand::rng()` was deprecated.  
   - **Solution:** Replaced with `rand::thread_rng()` and `gen_range()`.

3. **Visualization Issues**  
   - **Problem:** `textplots` sometimes generated unreadable output.  
   
##  Resources Used
- Rust Documentation: [https://doc.rust-lang.org/](https://doc.rust-lang.org/)
- `ndarray` Library: [https://docs.rs/ndarray/latest/ndarray/](https://docs.rs/ndarray/latest/ndarray/)
- `rand` Library: [https://docs.rs/rand/latest/rand/](https://docs.rs/rand/latest/rand/)
- AI Assistance: ChatGPT for debugging & optimization

##  Author
**Akhona Rilityana**  


