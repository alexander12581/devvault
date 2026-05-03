# devvault

Developer secrets & .env manager - cross-project environment variable management with security scanning.

## Features

- **Unified Environment Variable Management**: Manage all your project's environment variables through CLI commands
- **Encrypted Storage**: Securely store secrets using ChaCha20-Poly1305 encryption with Argon2 key derivation
- **Security Scanning**: Automatically scan code for hardcoded secrets (API keys, tokens, passwords)
- **Import/Export .env**: Import variables from .env files and export to .env format
- **Git Pre-commit Hook**: Prevent commits containing hardcoded secrets
- **Team Sharing**: Encrypted vault files can be safely committed to git repositories
- **Single Binary Distribution**: Compiled to a single executable, no runtime dependencies
- **Random Secret Generation**: Generate cryptographically secure random values for API keys and secrets

## Installation

### From Source

```bash
git clone https://github.com/alexander12581/devvault.git
cd devvault
cargo install --path .
```

### From Cargo

```bash
cargo install devvault
```

## Usage

### Initialize a Vault

```bash
devvault init
```

This creates a `.devvault/` directory with encrypted vault and configuration files.

### Set Environment Variables

```bash
# Set a variable with a specific value
devvault set API_KEY=your_secret_key
devvault set DATABASE_URL=postgres://user:pass@localhost/db

# Generate a random value for a variable (default length: 32 characters)
devvault set API_KEY --generate

# Generate a random value with custom length
devvault set SECRET_KEY --generate --length 64
```

### Get Environment Variables

```bash
devvault get API_KEY
```

### List All Variables

```bash
devvault list
```

### Remove Variables

```bash
devvault remove API_KEY
```

### Import from .env File

```bash
devvault import .env
```

### Export to .env Format

```bash
devvault export
```

### Scan for Hardcoded Secrets

```bash
# Scan current directory
devvault scan

# Scan specific path
devvault scan ./src
```

### Install Git Pre-commit Hook

```bash
devvault hook install
```

This installs a pre-commit hook that scans staged files for secrets before each commit.

### Uninstall Git Pre-commit Hook

```bash
devvault hook uninstall
```

## Security

### Encryption

- **Algorithm**: ChaCha20-Poly1305 (AEAD)
- **Key Derivation**: Argon2id (memory=64MB, iterations=3, parallelism=4)
- **Salt**: Random 16 bytes, stored in vault file header
- **Nonce**: Random 12 bytes, regenerated for each encryption

### Vault File Format

```
[salt:16][nonce:12][ciphertext+tag]
```

### Password Handling

Passwords are never stored on disk. They are only held in memory during operations and zeroized after use.

## Configuration

Configuration is stored in `.devvault/config.toml`:

```toml
version = 1
vault_path = "vault.enc"
scan_patterns = [
    "AKIA[0-9A-Z]{16}",
    "gh[pousr]_[A-Za-z0-9_]{36,}",
    # ... more patterns
]
exclude_patterns = [
    "node_modules",
    ".git",
    "target",
    "*.lock"
]
```

## Supported Secret Patterns

- AWS Access Keys
- GitHub Tokens
- Generic API Keys
- Private Keys (RSA, EC, OPENSSH)
- JWT Tokens
- Generic Secrets
- Database URLs

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Release Build

```bash
cargo build --release
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

MIT OR Apache-2.0

## Acknowledgments

- Inspired by the need for better secret management in development workflows
- Built with Rust for performance and security