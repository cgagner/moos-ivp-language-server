# Emacs Setup

## Install `moos-ivp-language-server`

### Source Code

Building the `moos-ivp-lanugage-server` is the preferred method for
developers that want to take advantage of the latest features. It requires
Rust to be installed on the system. See 
[Getting Started](https://www.rust-lang.org/learn/get-started) for instructions
on installing Rust for your system.

Once Rust is installed, the `moos-ivp-language-server` can be installed using:

```bash
git clone git@github.com:cgagner/moos-ivp-language-server.git
cd moos-ivp-language-server
cargo install --path .
```

Verify `moos-ivp-language-server` is installed using:

```bash
moos-ivp-language-server --help
```

### Binary Install

The `moos-ivp-language-server` automatically generates binary installation
as part of the release process. Navigate to the
[Releases](https://github.com/cgagner/moos-ivp-language-server/releases)
page and select the binary file from the `Assets` section that matches your
operating system and architecture. 

Once downloaded, extract the executable into a directory in your PATH. For
example, on Linux and Mac OS systems `~/.local/bin`.

Example:

```bash
mkdir -p ~/.local/bin
cp ~/Downloads/moos-ivp-language-server ~/.local/bin
chmod 755 ~/.local/bin/moos-ivp-language-server
echo "export PATH=${PATH}:~/.local/bin" >> ~/.bashrc
source ~/.bashrc
```

Verify `moos-ivp-language-server` is installed using:

```bash
moos-ivp-language-server --help
```

## Install `lsp-mode`

In order to take full advantage of the `moos-ivp-language-server`, it is 
recommended to install `lsp-mode`. This is a language server client. *NOTE:* 
the built-in language server client `Eglot` does not support semantic tokens.


Add the following to your `init.el` or `~/.emacs` file:

```lisp
;; Enable lsp-mode
(require 'package)
(add-to-list 'package-archives '("melpa" . "https://melpa.org/packages/") t)
(package-initialize)
(require 'lsp-mode)

;; Enable lsp-ui 
(package-install 'lsp-ui)
(add-hook 'lsp-mode-hook 'lsp-ui-mode)

;; Define moos-mode
(define-derived-mode moos-mode prog-mode "moos"
  "A simple mode for moos."
  ;; Set comment start and end
  (setq-local comment-start "//")
  (setq-local comment-end "")
  ;; Allow whitespace after // for skipping
  (setq-local comment-start-skip "//\\s-*")

  ;; Add font lock keywords for highlighting
  (font-lock-add-keywords
   nil
   '(("//.*" . font-lock-comment-face)  ;; Single-line comments
     ("/\\*.*?\\*/" . font-lock-comment-face))) ;; Multi-line comments

  ;; Ensure that font-lock mode is active
  (font-lock-mode 1))

;; Register moos language server with the custom mode
(lsp-register-client
 (make-lsp-client
  :new-connection (lsp-stdio-connection "moos-ivp-language-server")
  :major-modes '(moos-mode)
  :server-id 'moos-lsp))

(defun moos-language-setup ()
  (lsp)  ;; Start LSP manually in this mode
  (setq lsp-semantic-tokens-enable t))  ;; Ensure semantic tokens are enabled

;; Setup moos-mode to open on moos, behavior, and plug files
(add-to-list 'auto-mode-alist '("\\.moos\\'" . moos-mode))
(add-to-list 'auto-mode-alist '("\\.bhv\\'" . moos-mode))
(add-to-list 'auto-mode-alist '("\\.plug\\'" . moos-mode))
(add-to-list 'auto-mode-alist '("\\.def\\'" . moos-mode))
(add-hook 'moos-mode-hook 'moos-language-setup)

;; Enable global font lock mode if not already enabled
(global-font-lock-mode 1)

;; Uncomment to enable lsp logging. This should only be needed if there is 
;; something wrong with the LSP.
;(setq lsp-log-io t)

(with-eval-after-load 'lsp-mode
  (add-to-list 'lsp-language-id-configuration '(moos-mode . "moos")))

;; Semantic tokens (semantic highlighting)
(setq lsp-semantic-tokens-enable t)

;; Enable inlay hints globally or in specific programming modes
(add-hook 'lsp-mode-hook #'lsp-inlay-hints-mode)

;; Enable inlay hints (for type annotations, etc.)
(setq lsp-inlay-hints-enable t)

;; Enable code folding
(setq lsp-enable-folding t)
```

1. Open Emacs
2. Enter the command `M-x` package-refresh-contents `RET`
3. Enter the command `M-x` package-install `RET` lsp-mode `RET`

For new Emacs users, `M-x` is equivalent to hitting the `<esc>` key followed by the `x` key.

### References

* [Install MEPLA](https://melpa.org/#/getting-started)
* [Install lsp-mode](https://emacs-lsp.github.io/lsp-mode/page/installation/)
