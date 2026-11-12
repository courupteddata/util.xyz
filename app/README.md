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
URL=$(curl -s https://api.github.com/repos/gchq/CyberChef/releases/latest |
  jq -r '[.assets[] | select(.name | test("^CyberChef_.*\\.zip$"))] | first | .browser_download_url')

rm -rf cyberchef && mkdir cyberchef
curl -fsSL -o cc.zip "$URL" && unzip -q cc.zip -d cyberchef && rm cc.zip
mv "$(find cyberchef -maxdepth 1 -name 'CyberChef*.html' | head -n1)" cyberchef/index.html

dx serve
```

Local docker build
```bash
URL=$(curl -s https://api.github.com/repos/gchq/CyberChef/releases/latest |
  jq -r '[.assets[] | select(.name | test("^CyberChef_.*\\.zip$"))] | first | .browser_download_url')
docker build --build-arg CYBERCHEF_URL="$URL" -t main-page:local .
docker run --rm -p 8080:8080 main-page:local
```

Set `CYBERCHEF_DIR` to use a different CyberChef location.
