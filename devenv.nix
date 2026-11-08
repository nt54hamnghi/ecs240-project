{
  pkgs,
  ...
}:

{

  packages = with pkgs; [
    git
    just
    # texlab
    texliveMedium
    zathura
  ];

  languages = {
    rust = {
      enable = true;
      channel = "stable"; # default
    };
  };
}
