# Aston M. Shand — Portfolio

A Rust + Axum + Askama portfolio site. Same stack as
[wattfinds.com](https://wattfinds.com). Single-page, responsive, dark/light
theme, animated reveals, filterable project grid.

## Local development

```bash
# run the dev server
cargo run --release
# open http://localhost:8080

# or on a custom port
PORT=3000 cargo run --release
```

## Build a static copy (for free static hosting)

```bash
cargo run --release --bin build_static
# outputs ./dist/ with index.html + static/ ready to deploy anywhere
```

## Free hosting options

### Option 1 — Static hosting (truly free, no sleeping, recommended)

After `cargo run --release --bin build_static`, upload `dist/` to any of:

| Host              | Free tier                                   | Setup                      |
|-------------------|---------------------------------------------|----------------------------|
| **Cloudflare Pages** | Unlimited sites, unlimited bandwidth    | Connect GitHub → set build command `cargo run --release --bin build_static` and output dir `dist` |
| **GitHub Pages**  | 1 GB storage, 100 GB/month bandwidth, custom domain | Push `dist/` to `gh-pages` branch or use an action |
| **Netlify**       | 100 GB/month bandwidth, free SSL          | Connect GitHub → build command `cargo run --release --bin build_static`, publish dir `dist` |
| **Vercel**        | 100 GB/month bandwidth                    | Same idea; needs Rust build setup |

**Recommended: Cloudflare Pages** — unlimited bandwidth and the fastest edge
network. Point your custom domain at it for free.

### Option 2 — Dynamic Rust server (shows off full Rust stack)

Deploy the Axum server directly. Best options:

| Host            | Free tier                                     | Notes                         |
|-----------------|-----------------------------------------------|-------------------------------|
| **Fly.io**      | 3 shared-cpu-1x VMs, 256 MB RAM, 160 GB/mo   | Best Rust support; `fly.toml` included |
| **Shuttle.dev** | Free tier, Rust-native, auto-deploy from GitHub | Needs code tweak for shuttle runtime |
| **Render**      | Free web service (sleeps after 15 min idle)  | Simple Dockerfile deploy     |
| **Railway**     | $5 credit/month                               | Simple Dockerfile deploy     |

**Recommended: Fly.io** — generous free tier, no sleeping with
`auto_stop_machines`, runs the real Axum binary.

#### Deploy to Fly.io

```bash
# one time
curl -L https://fly.io/install.sh | sh
fly auth signup

# deploy (uses included fly.toml and Dockerfile)
cd ~/portfolio
fly launch --no-deploy --copy-config  # accept existing fly.toml
fly deploy
```

Your site will be live at `https://aston-portfolio.fly.dev`.

## Architecture

```
portfolio/
├── Cargo.toml
├── Dockerfile           # for Fly.io / Render / Railway
├── fly.toml             # Fly.io deployment config
├── src/
│   ├── main.rs          # Axum server entry
│   ├── projects.rs      # Project data
│   └── bin/
│       └── build_static.rs  # Static site generator
├── templates/
│   ├── index.html       # Askama template (main page)
│   └── 404.html
└── static/
    ├── css/style.css
    └── js/main.js
```

## Editing content

All project data lives in `src/projects.rs`. Add, edit, or remove
projects there and rebuild.

Personal info (name, email, phone, bio) lives directly in
`templates/index.html`.
