# Quick start

## 1. Install

=== "mise"

    ```sh
    mise use --pin github:rbx-forge/rbx-ops
    ```

=== "Rokit"

    ```sh
    rokit add rbx-forge/rbx-ops
    ```

## 2. Get a read-only key

In the Creator Hub, **Open Cloud > API Keys**: a key restricted to your experience with `universe:read`. Pass it through the environment:

```sh
export RBX_API_KEY="..."
```

## 3. Look at your servers

```sh
rbx-ops servers versions --universe-id 9876543210 --place-id 1234567890
rbx-ops servers list --universe-id 9876543210 --place-id 1234567890
```

Servers belong to a place, so both ids are needed. With an `rbxplace.toml` in the project, `--env prod` replaces them: see [Configuration](reference/configuration.md).

## 4. Before writing anything

Read the [safety model](reference/safety.md), and make a separate key with only the write scope you need ([scopes](reference/scopes.md)). Writes are dry runs until `--apply`:

```sh
rbx-ops ban add builderman --reason "exploit: fly hack" --duration 7d --env prod           # shows the payload
rbx-ops ban add builderman --reason "exploit: fly hack" --duration 7d --env prod --apply   # asks, then sends
```
