# Part2

## 1. storage model
- For detailed explanation: [official doc](https://solana.com/zh/docs/core/accounts)

## 2. runtime memory

### 2.1 datatype or access not allowed
- Types require dynamic memory allocation, which is not supported in Solana programs.

- Solana programs don’t have access to an operating system:
    - Any IO-related types (File, TcpStream, etc.)
    - Nondeterministic system time query

### 2.2 dynamic datatype simulation

- HashMap is natively implemented in solana because heap memory allocation is not allowed.
    - Demo
        The only difference against plain instruction is the parameter ```key``` passed to generate ```seeds``` as part of constraint
    ```rust
    #[derive(Accounts)]
    #[instruction(key: String)] // <-- makes `key` available in constraints
    pub struct CreateUser<'info> {
        #[account(
            init,
            ...
            seeds = [&key.as_bytes().as_ref()],
            ...
        )]
        pub user_account: Account<'info, User>,

        ...
    }
    ```
- It can be simulated by fixed buffer or PDA mapping address to account data.
    - Key: 
    For PDA mapping, any predefined input (key) can be used as a seed to deterministically derive a PDA address with a given program ID. 
    - Value:
    The value associated with the key is stored in the on-chain account data, defined by a fixed-size struct that is allocated when the account is initialized.

