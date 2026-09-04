# MeshelDesk Windows build

The MeshelDesk client is configured at build time with:

- ID server: `remote.meshel.cn:21116`
- Relay server: `remote.meshel.cn:21117`
- Server public key: `oVvhm7iAfi5UZ3j9++b7ysrtMEvyKBz+iLgEzSzrzgE=`
- Runtime product name: `MeshelDesk`

The server fields are locked, their settings entry is hidden, and upstream automatic updates are disabled. UAC elevation and service installation remain available.

## Build in GitHub Actions

1. Open the repository's **Actions** tab.
2. Select **Build MeshelDesk for Windows**.
3. Choose **Run workflow** on the `master` branch.
4. After the job succeeds, download the `MeshelDesk-1.5.0-windows-x86_64` artifact.
5. Extract the artifact and verify `SHA256SUMS.txt` before distributing `MeshelDesk-1.5.0-windows-x86_64-portable.exe`.

No repository secrets are needed for this unsigned build. Windows SmartScreen may warn about the executable until it is signed with a trusted code-signing certificate and has accumulated reputation.

## Server requirements

`remote.meshel.cn` must resolve to the RustDesk server, with TCP ports `21115` through `21119` and UDP port `21116` published directly. These are RustDesk protocol ports; do not configure them as an HTTP reverse proxy in 1Panel.

The matching `id_ed25519` private key must remain only in the server data volume. If the server key pair is replaced, update `SERVER_KEY` in `src/mesheldesk.rs` and rebuild the client.
