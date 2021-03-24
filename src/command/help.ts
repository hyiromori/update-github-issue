// フォーマットは gh コマンドを参考にしています。
const usage = () =>
  [
    "",
    "USAGE",
    "  github-issue <command> [options] [...GitHub Issue URLs]",
    "",
    "CORE COMMANDS",
    "  create",
    "  update",
    "  help",
    "",
    "OPTIONS",
    "  --title <Create Issue Title>",
    "  --epic-url <GitHub Issue URL>",
    "  --label <Label Name>",
    "  --owner <GitHub Repository Owner>",
    "  --repository <GitHub Repository Name>",
    "  --urls <GitHub Issue URL>",
    "  --workspace <ZenHub Workspace Name>",
    "  --pipeline <ZenHub Pipeline Name>",
    "  --verbose",
    "",
    "LEARN MORE",
    "  Read the manual at https://github.com/hyiromori/github-update-issue/blob/main/README.md",
    "",
  ].join("\n");

export const help = (): void => {
  console.log(usage());
};
