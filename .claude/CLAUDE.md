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

## Feature Development — Multi-Agent Workflow

When asked to create a new feature, orchestrate work across three specialized agents running in an isolated git worktree branch. This enforces the strict TDD workflow above with clear separation of concerns.

### 1. Branch Setup

- Create a new git worktree branch for the feature (e.g. `feat/<feature-name>`).
- All agent work happens on this branch, keeping `main` clean.

### 2. test-developer Agent

- **Responsibility:** Write tests only.
- Write unit and/or integration tests that describe the expected behavior of the feature.
- Run the tests and verify they **fail** (red). Do not proceed until all new tests are red.
- **Must not** write any implementation code.
- Commit changes with a `test:` conventional commit message.

### 3. feature-developer Agent

- **Responsibility:** Write implementation code only.
- Write the minimum code needed to make all tests written by `test-developer` pass.
- Run the tests and verify they all **pass** (green).
- **Must not** modify any tests.
- Commit changes with a `feat:` conventional commit message.

### 4. architect-developer Agent

- **Responsibility:** Review and improve implementation code.
- Refactor for clarity, simplicity (KISS), and good practices.
- Run the full test suite and verify all tests still **pass**.
- **Must not** modify any tests.
- Commit changes with a `refactor:` conventional commit message.

### 5. Finalize

- Push the feature branch to the remote.
- Create a pull request targeting `main`.
