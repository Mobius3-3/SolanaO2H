# Part1

## 1. macro - account

### 1.1 #[account]

1. Explanation:
    ```#[account]``` is procedural macro which generates codes from raw codes under its function. It includes:

    - serilize struct type data as data blob and deserilize reversely

    - print struct in debug mode

2. Demo

    ```rust
    #[account]
    pub struct MyStorage {
        value1: u8,
    }
    ```

    Based on struct ```MyStorage``` and macro ```#[account]```, three functions can be realized:


    - Serialize a MyStorage instance into bytes

    - Deserialize from bytes

    - Print it for debug

    ```rust
    fn main() {
        // ✅ Create an instance
        let storage = MyStorage { value1: 42 };

        // ✅ Serialize with discriminator
        let mut serialized: Vec<u8> = Vec::new();
        storage.try_serialize(&mut serialized).unwrap();

        // ✅ Deserialize from bytes
        let mut bytes_slice = serialized.as_slice();
        let deserialized = MyStorage::try_deserialize(&mut bytes_slice).unwrap();

        // ✅ Print result
        println!("Deserialized struct: {:?}", deserialized);
    }
    ```

