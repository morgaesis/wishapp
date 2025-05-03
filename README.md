# WishApp

WishApp is a web application that allows users to create and manage wishlists

The infrastructure is designed to be cloud-native, utilizing AWS services within the free tier. CI/CD is managed using GitHub Actions.

## Development

### Git Hooks Setup

To install the pre-push hook that runs code quality checks:

```bash
cp .githooks/pre-push .git/hooks/pre-push
chmod +x .git/hooks/pre-push
```

The hook will:

- Check code formatting with Prettier
- Validate YAML files
- Run actionlint when workflow files change

## License

This project is licensed under the MIT License.
