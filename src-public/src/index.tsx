import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { listen } from "@tauri-apps/api/event";
import { setup } from "goober";
import {
  Form,
  FormItem,
  Label,
  PrimaryButton,
  SubmitButton,
  TextInput,
} from "./ui";
import { Progress } from "./progress";

setup(React.createElement);

function App() {
  const [host, setHost] = useState("");
  const [port, setPort] = useState("");
  const [login, setLogin] = useState("");
  const [password, setPassword] = useState("");
  const [outPath, setOutPath] = useState("");
  const [isOutputButtonHovered, setIsOutputButtonHovered] = useState(false);
  const [isExporting, setIsExporting] = useState(false);
  const [progress, setProgress] = useState(0);

  const disabled = !host || !port || !login || !password || !outPath;

  async function onSubmit(e: React.FormEvent) {
    e.preventDefault();

    if (disabled) {
      return;
    }

    setIsExporting(true);
    try {
      invoke("run_email_export", {
        host,
        port: parseInt(port),
        login,
        password,
        outPath,
      });
    } catch (e) {
      console.error(e);
      alert("There is an error while exporting. Please try again.");
      setIsExporting(false);
    }
  }

  async function openOutPathSelector() {
    const result = await open({
      directory: true,
      defaultPath: outPath || undefined,
    });
    if (typeof result === "string" && result.length) {
      setOutPath(result);
      setIsOutputButtonHovered(false);
    }
  }

  useEffect(() => {
    listen<number>("progress", (e) => {
      setProgress((e.payload || 0) * 100);
    });

    listen("success", () => {
      setProgress(0);
      setIsExporting(false);
    });
  }, []);

  return isExporting ? (
    <Progress value={progress} />
  ) : (
    <Form onSubmit={onSubmit}>
      <FormItem>
        <Label htmlFor="host">IMAP Host</Label>
        <TextInput
          id="host"
          name="host"
          type="text"
          placeholder="imap.example.com"
          value={host}
          onChange={(e) => setHost(e.target.value)}
          required
        />
      </FormItem>

      <FormItem>
        <Label htmlFor="port">Port</Label>
        <TextInput
          id="port"
          name="port"
          type="number"
          placeholder="993"
          value={port}
          onChange={(e) => setPort(e.target.value)}
          required
        />
      </FormItem>

      <FormItem>
        <Label htmlFor="login">Login</Label>
        <TextInput
          id="login"
          name="login"
          type="text"
          value={login}
          onChange={(e) => setLogin(e.target.value)}
          required
        />
      </FormItem>

      <FormItem>
        <Label htmlFor="host">Password</Label>
        <TextInput
          id="password"
          name="password"
          type="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          required
        />
      </FormItem>

      <FormItem
        onMouseEnter={() => setIsOutputButtonHovered(true)}
        onMouseLeave={() => setIsOutputButtonHovered(false)}
      >
        {!outPath || isOutputButtonHovered ? (
          <PrimaryButton type="button" onClick={openOutPathSelector}>
            Select output folder
          </PrimaryButton>
        ) : (
          <PrimaryButton type="button" disabled>
            {outPath}
          </PrimaryButton>
        )}
      </FormItem>

      <FormItem>
        <SubmitButton type="submit" disabled={disabled}>
          Export
        </SubmitButton>
      </FormItem>
    </Form>
  );
}

ReactDOM.render(<App />, document.getElementById("app"));
