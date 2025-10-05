# Bazooka 🚀

An intuitive, blazing-fast, and customizable application launcher built with Rust and GTK4. <br/><br/>
<img width="1919" height="1049" src="https://github.com/user-attachments/assets/7334589a-640b-478a-b9f6-ced5df611b6d" />

-----

## Features

  * **Fuzzy Search:** Fuzzy find and launch applications by typing just a few characters of their name. Powered by SkiaMatcherV2.

  * **Calculator:** Perform calculations directly in the search bar. Press **Enter** to copy the result to your clipboard.

  * **Themable with CSS:** Customize every aspect of Bazooka's appearance using simple CSS. Make it blend perfectly with your desktop theme and personal workflow.

  * **Desktop Actions:** Access application-specific actions directly from the launcher (e.g., "Open a New Private Window" for your browser or "New Document" for your office suite).

  * **Plugin Support** *(Coming Soon\!)*: Expand Bazooka's capabilities with custom plugins. Search files, manage browser tabs, interact with web services, and more.

-----

## Installation

You can install Bazooka either from a pre-compiled binary or by building it from the source.

### 1\. Pre-compiled Binary (Recommended)

This is the easiest way to get started.

1.  Download the latest binary from the [**Releases Page**](https://www.google.com/search?q=https://github.com/YOUR_USERNAME/bazooka/releases).
2.  Make the binary executable:
    ```bash
    chmod +x bazooka
    ```
3.  Move the binary to a directory in your system's PATH:
    ```bash
    sudo mv bazooka /usr/local/bin/
    ```

### 2\. Build from Source

If you prefer to build it yourself, you'll need the Rust toolchain and the GTK4 development libraries.

1.  **Install Dependencies:**

      * **Arch Linux:**
        ```bash
        sudo pacman -S rustup gtk4
        ```
      * **Debian / Ubuntu:**
        ```bash
        sudo apt update
        sudo apt install rustc cargo libgtk-4-dev
        ```
      * **Fedora:**
        ```bash
        sudo dnf install rustc cargo gtk4-devel
        ```

2.  **Clone and Build the Project:**

    ```bash
    git clone https://github.com/YOUR_USERNAME/bazooka.git
    cd bazooka
    cargo build --release
    ```

    The executable will be located at `target/release/bazooka`. You can then move it into your PATH.

-----

## Customization

You can customize the look and feel of Bazooka by creating a custom stylesheet.

1.  Create the configuration directory:
    ```bash
    mkdir -p ~/.config/bazooka/
    ```
2.  Create a stylesheet at `~/.config/bazooka/style.css`. Bazooka will automatically load this file on startup.

**Example `style.css`:**

```css
/* Make the window background semi-transparent and rounded */
window.background {
  background-color: rgba(40, 40, 40, 0.9);
  border-radius: 12px;
}

/* Style the search entry */
.search-entry {
  background-image: none;
  background-color: #333;
  color: white;
  border: 1px solid #555;
  border-radius: 8px;
  font-size: 1.8rem;
  min-height: 50px;
}

/* Style for a selected result row */
listbox row.result-row:selected {
  background-image: none;
  background-color: #4a90e2;
  border-radius: 8px;
}

/* Make the text in the selected row white */
listbox row.result-row:selected label {
  color: white;
}
```
