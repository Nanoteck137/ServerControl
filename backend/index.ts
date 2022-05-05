import express from "express";
import dotenv from "dotenv";
import { ParseServer } from "parse-server";

dotenv.config();

function err(message: string) {
    throw new Error(message);
}

const databaseUri   = process.env.BACKEND_PARSE_DATABASE_URI  || err("'BACKEND_PARSE_DATABASE_URI' not set");
const appId         = process.env.BACKEND_PARSE_APP_ID        || err("'BACKEND_PARSE_APP_ID' not set");
const masterKey     = process.env.BACKEND_PARSE_MASTER_KEY    || err("'BACKEND_PARSE_MASTER_KEY' not set");
const serverURL     = process.env.BACKEND_PARSE_SERVER_URL    || err("'BACKEND_PARSE_SERVER_URL' not set");

const parseConfig = {
    databaseURI: databaseUri,
    appId: appId,
    masterKey: masterKey,
    serverURL: serverURL,
    liveQuery: {
      classNames: []
    },
    directAccess: true,
    enforcePrivateUsers: true,
};

const parseApi = new ParseServer(parseConfig);

const app = express();
const port = process.env.SERVER_PORT || 3000;

app.get("/", (req, res) => {
    res.send("Hello World");
});

app.use("/parse", parseApi);

app.listen(port, () => {
    console.log(`Backend server is running on port ${port}`)
});
