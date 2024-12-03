import { invoke } from "@tauri-apps/api/core";
import {
  type EventCallback,
  type EventName,
  type Options,
  type UnlistenFn,
  listen,
} from "@tauri-apps/api/event";

export async function ping(value: string): Promise<string | null> {
  return await invoke<{ value?: string }>("plugin:klaver|ping", {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null));
}

export async function open(path: string): Promise<Vm> {
  return await invoke<number>("plugin:klaver|vm_open", {
    path,
  }).then((r) => new Vm(r));
}

class Vm {
  #id: number;

  constructor(id: number) {
    this.#id = id;
  }

  listen(cb: (level: string, message: string) => void): UnlistenFn {
    return listenSync<{ vm: number; message: string; level: string }>(
      "klaver://console",
      (event) => {
        if (event.payload.vm !== this.#id) {
          return;
        }
        cb(event.payload.level, event.payload.message);
      }
    );
  }

  async typings(): Promise<Record<string, string>> {
    return await invoke<Record<string, string>>("plugin:klaver|vm_typings", {
      vm: this.#id,
    });
  }

  async exec(code: string) {
    return await invoke<unknown>("plugin:klaver|vm_exec", {
      vm: this.#id,
      code,
    });
  }

  async close() {
    await invoke("plugin:klaver|vm_close", { vm: this.#id });
  }

  async eval(name: string, code: string) {
    return await invoke<number>("plugin:klaver|vm_eval_module", {
      vm: this.#id,
      name,
      code,
    });
  }
}

function listenSync<T>(
  event: EventName,
  handler: EventCallback<T>,
  options?: Options
): UnlistenFn {
  const ret: { value: UnlistenFn | boolean } = {
    value: false,
  };

  listen(event, handler, options).then((res) => {
    if (ret.value) res();
    ret.value = res;
  });

  return () => {
    if (typeof ret.value === "function") {
      ret.value();
    } else {
      ret.value = true;
    }
  };
}
