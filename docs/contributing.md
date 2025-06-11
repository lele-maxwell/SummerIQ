# Contributing to ZipMind

Thank you for your interest in contributing to ZipMind! This guide will help you get started with the contribution process.

## Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](./code-of-conduct.md). Please read it before contributing.

## How to Contribute

### 1. Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/yourusername/zipmind.git
   cd zipmind
   ```

### 2. Set Up Development Environment

Follow the [Getting Started](./getting-started.md) guide to set up your development environment.

### 3. Create a Branch

Create a new branch for your feature or bug fix:
```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

### 4. Make Changes

1. **Follow Coding Standards**
   - Use consistent formatting
   - Follow language-specific style guides
   - Write clear commit messages

2. **Write Tests**
   - Add unit tests for new features
   - Update existing tests if needed
   - Ensure all tests pass

3. **Update Documentation**
   - Update relevant documentation
   - Add comments for complex code
   - Update README if necessary

### 5. Commit Changes

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```bash
# Format
<type>(<scope>): <description>

# Examples
feat(auth): add password reset functionality
fix(upload): handle large file uploads
docs(api): update authentication endpoints
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### 6. Push Changes

```bash
git push origin feature/your-feature-name
```

### 7. Create Pull Request

1. Go to the [Pull Requests](https://github.com/yourusername/zipmind/pulls) page
2. Click "New Pull Request"
3. Select your branch
4. Fill in the PR template
5. Submit the PR

## Development Guidelines

### Code Style

#### Rust (Backend)

1. **Formatting**
   ```bash
   cargo fmt
   ```

2. **Linting**
   ```bash
   cargo clippy
   ```

3. **Documentation**
   ```rust
   /// Function documentation
   /// 
   /// # Arguments
   /// * `param` - Parameter description
   /// 
   /// # Returns
   /// Description of return value
   pub fn function(param: Type) -> Result<Type> {
       // Implementation
   }
   ```

#### TypeScript (Frontend)

1. **Formatting**
   ```bash
   npm run format
   # or
   yarn format
   ```

2. **Linting**
   ```bash
   npm run lint
   # or
   yarn lint
   ```

3. **Type Definitions**
   ```typescript
   interface Props {
     /** Property description */
     prop: Type;
   }
   ```

### Testing

#### Backend Tests

1. **Unit Tests**
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_function() {
           // Test implementation
       }
   }
   ```

2. **Integration Tests**
   ```rust
   #[cfg(test)]
   mod integration_tests {
       use super::*;

       #[tokio::test]
       async fn test_endpoint() {
           // Test implementation
       }
   }
   ```

#### Frontend Tests

1. **Component Tests**
   ```typescript
   describe('Component', () => {
     it('should render correctly', () => {
       // Test implementation
     });
   });
   ```

2. **Integration Tests**
   ```typescript
   describe('Feature', () => {
     it('should work end-to-end', () => {
       // Test implementation
     });
   });
   ```

### Documentation

1. **Code Documentation**
   - Document public APIs
   - Add examples where helpful
   - Keep documentation up to date

2. **Project Documentation**
   - Update README.md
   - Add/update API documentation
   - Document configuration options

## Review Process

1. **Code Review**
   - All PRs require at least one review
   - Address review comments
   - Keep PRs focused and small

2. **CI/CD Checks**
   - All tests must pass
   - Code must be formatted
   - No linting errors

3. **Merge Process**
   - Squash commits if needed
   - Use conventional commit messages
   - Update documentation

## Feature Requests

1. **Before Submitting**
   - Check existing issues
   - Search for similar features
   - Consider implementation complexity

2. **Submitting**
   - Use the feature request template
   - Provide clear description
   - Include use cases

## Bug Reports

1. **Before Submitting**
   - Check existing issues
   - Try to reproduce the bug
   - Check documentation

2. **Submitting**
   - Use the bug report template
   - Include steps to reproduce
   - Provide error messages
   - Add screenshots if relevant

## Release Process

1. **Versioning**
   - Follow [Semantic Versioning](https://semver.org/)
   - Update version numbers
   - Update changelog

2. **Release Steps**
   - Create release branch
   - Run full test suite
   - Update documentation
   - Create release tag
   - Deploy to production

## Community

1. **Communication**
   - Use GitHub Issues
   - Join our Discord server
   - Follow our blog

2. **Support**
   - Help other contributors
   - Answer questions
   - Review PRs

## License

By contributing, you agree that your contributions will be licensed under the project's [LICENSE](../LICENSE) file.

## Acknowledgments

Thank you for contributing to ZipMind! Your contributions help make the project better for everyone. 