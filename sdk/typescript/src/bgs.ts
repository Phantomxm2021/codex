import { BgsOptions } from "./bgsOptions";
import { BgsExec } from "./exec";
import { Thread } from "./thread";
import { ThreadOptions } from "./threadOptions";

/**
 * Bgs is the main class for interacting with the Bgs agent.
 *
 * Use the `startThread()` method to start a new thread or `resumeThread()` to resume a previously started thread.
 */
export class Bgs {
  private exec: BgsExec;
  private options: BgsOptions;

  constructor(options: BgsOptions = {}) {
    const { bgsPathOverride, env, config, configOverrides } = options;
    this.exec = new BgsExec(bgsPathOverride, env, config, configOverrides);
    this.options = options;
  }

  /**
   * Starts a new conversation with an agent.
   * @returns A new thread instance.
   */
  startThread(options: ThreadOptions = {}): Thread {
    return new Thread(this.exec, this.options, options);
  }

  /**
   * Resumes a conversation with an agent based on the thread id.
   * Threads are persisted in ~/.bgs/sessions.
   *
   * @param id The id of the thread to resume.
   * @returns A new thread instance.
   */
  resumeThread(id: string, options: ThreadOptions = {}): Thread {
    return new Thread(this.exec, this.options, options, id);
  }
}
