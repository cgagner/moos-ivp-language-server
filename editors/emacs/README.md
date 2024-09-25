# Emacs Setup

## Install `lsp-mode`

In order to take full advantage of the `moos-ivp-language-server`, it is 
recommended to install `lsp-mode`. This is a language server client. NOTE: 
the built-in language server client `Eglot` does not support semantic tokens.


Add the following to your `init.el` or `~/.emacs` file:

```lisp
(require 'package)
(add-to-list 'package-archives '("melpa" . "https://melpa.org/packages/") t)
;; Comment/uncomment this line to enable MELPA Stable if desired.  See `package-archive-priorities`
;; and `package-pinned-packages`. Most users will not need or want to do this.
;;(add-to-list 'package-archives '("melpa-stable" . "https://stable.melpa.org/packages/") t)
(package-initialize)
```

1. Open Emacs
2. Enter the command `M-x` package-refresh-contents `RET`
3. Enter the command `M-x` package-install `RET` lsp-mode `RET`



## TODO: Update this section


```lisp
(require 'package)
(add-to-list 'package-archives '("melpa" . "https://melpa.org/packages/") t)
(package-initialize)
(require 'lsp-mode)

(add-to-list 'load-path "~/repos/moos-ivp/editor-modes/")
  (require 'moos-mode)

(with-eval-after-load 'lsp-mode
  (add-to-list 'lsp-language-id-configuration
               '(moos-mode . "moos"))

  (lsp-register-client
   (make-lsp-client :new-connection (lsp-stdio-connection "moos-ivp-language-server")
                    :activation-fn (lsp-activate-on "moos")
                    :server-id 'moos-ivp-language-server)))


(use-package lsp-mode
  :commands lsp
  :hook
  (moos-mode . lsp))

;; Semantic tokens (semantic highlighting)
(setq lsp-semantic-tokens-enable t)

;; Enable inlay hints globally or in specific programming modes
(add-hook 'lsp-mode-hook #'lsp-inlay-hints-mode)

;; Enable inlay hints (for type annotations, etc.)
(setq lsp-inlay-hints-enable t)

;; Enable code folding
(setq lsp-enable-folding t)

```

TODO:
  - [ ] Add instructions for installing the server
  - [ ] Update so we don't need the existing moos-ivp editor modes.


### References

* [Install MEPLA](https://melpa.org/#/getting-started)
* [Install lsp-mode](https://emacs-lsp.github.io/lsp-mode/page/installation/)
