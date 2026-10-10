import { Hono } from 'hono'
import pwshScript from "./update.ps1";

const app = new Hono()

app.get('/', (c) => {

if (c.req.header("User-Agent")?.includes("PowerShell")) {
    c.header('Content-Type', 'text/plain')
    return c.text(pwshScript);
}


  return c.redirect('https://github.com/artifishvr/vrc_adblock')
})

export default app
