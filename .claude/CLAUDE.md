# Budgy

## Development Workflow — Strict TDD

When developing any new feature, always follow this sequence:

1. **Write tests first** — Create unit and/or integration tests for the feature before writing any implementation code.
2. **Verify red** — Run the tests and confirm they fail. Do not proceed until you have failing tests.
3. **Freeze tests** — From this point on, do not modify the tests you just wrote. Only write implementation code.
4. **Implement** — Write the minimum code needed to make the tests pass.
5. **Verify green** — Run the tests and confirm they all pass.
6. **Review** — Only present the work for review once all tests are green.

### Rules

- Never modify a test to make it pass. If a test is red, fix the implementation, not the test.
- Tests may only be changed in a separate request where requirements themselves change.
- Always run the full test suite before asking for review, not just the new tests.
- Always use [Conventional Commits](https://www.conventionalcommits.org/) for commit messages (e.g. `feat:`, `fix:`, `test:`, `refactor:`, `docs:`, `chore:`).
