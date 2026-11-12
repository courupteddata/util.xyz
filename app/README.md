# main-page

Dioxus fullstack landing page for hosted services.

## Routes
- `/` -> landing page listing hosted services
- `/cyberchef` -> CyberChef embedded in the site layout
- `/cyberchef-app/` -> static CyberChef release (full screen)
- `/health` -> health check

## Adding a service
1. Add a route variant to `Route` in `src/main.rs`.
2. Add an entry to `services()`. It then appears on the landing page and in the nav.
3. If it's static files, mount them in `main()` with `nest_service` and fetch them in the `Dockerfile`.

## Local development

```bash
V=$(curl -s https://api.github.com/repos/gchq/CyberChef/releases/latest | jq -r .tag_name)
curl -fsSL -o cc.zip "https://github.com/gchq/CyberChef/releases/download/$V/CyberChef_$V.zip"
mkdir -p cyberchef && unzip -q cc.zip -d cyberchef && mv "cyberchef/CyberChef_$V.html" cyberchef/index.html
rm cc.zip

dx serve
```

Set `CYBERCHEF_DIR` to use a different CyberChef location.
