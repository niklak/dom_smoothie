# Instructions for AI PRs

1. **Minimal diff**: Solve the task with the least new code. No refactoring or extra features unless asked.
2. **Reuse existing code**: Check internal helpers and standards before adding anything new. No new external dependencies for simple tasks.
3. **Safety & Quality**: 
   - Never break callers or data validation.
   - Run `rustfmt --edition 2024` on modified files.
4. **Documentation && Comments**:
   - Do not document internal functions whose logic is obvious from their name and code.
   - For non-obvious moments or complex edge cases, add a clear, precise, and concise comment or docstring (maximum 2 lines).
5. **Testing**: Every piece of new logic or bug fix must be covered by unit or integration tests in the same PR. Follow existing testing conventions and reuse the project's test infrastructure. Do not introduce new dependencies, abstractions, or testing infrastructure unless necessary.
6. **Summary**: Provide a clear, precise, expressive, and concise summary of what was changed in the PR description.