set nocompatible
filetype off

set rtp+=~/.vim/bundle/Vundle.vim
call vundle#begin()

Plugin 'VundleVim/Vundle.vim'
Plugin 'airblade/vim-gitgutter'
Plugin 'tpope/vim-surround'
Plugin 'scrooloose/nerdtree'
Plugin 'scrooloose/syntastic'
Plugin 'rust-lang/rust.vim'
Plugin 'Valloric/YouCompleteMe'
Plugin 'morhetz/gruvbox'
Plugin 'itchyny/lightline.vim'
call vundle#end()

filetype plugin indent on
syntax on
set tabstop=4
set shiftwidth=4
set expandtab
set number
set encoding=utf-8 " For YouCompleteMe
set laststatus=2 " For lightline.vim
set t_u7= " Weird workaround
set t_RV=

let g:ycm_confirm_extra_conf = 0

colorscheme gruvbox
set background=dark

set statusline+=%#warningmsg#
set statusline+=%{SyntasticStatuslineFlag()}
set statusline+=%*

let g:syntastic_always_populate_loc_list = 1
let g:syntastic_auto_loc_list = 1
let g:syntastic_check_on_open = 0
let g:syntastic_check_on_wq = 0

" COMP590 Specific Keyboard Shortcuts

" The capital C in <C-...> is control
" <CR> is carriage return (pressing enter)

" Ctrl-o toggles the open file menu
map <C-o> :NERDTreeToggle<CR>

" Ctrl-x saves all, saves vim session, quits
map <C-x> :wa<CR>:mksession!<CR>:qa<CR>

" Shift-T saves and runs rust test
nmap T :wa<CR>:RustTest<CR>

" Ctrl-t saves all files and runs all tests
nmap <C-t> :wa<CR>:RustTest!<CR>

" Ctrl-G Go! Save and run your program.
nmap <C-g> :wa<CR>:RustRun 

" Scroll within 15 lines of bottom / top
set scrolloff=15

" Avoid U/D/L/R keys
map <Up> :echo "Use k to move up."<CR>
map <Down> :echo "Use j to move down."<CR>
map <Left> :echo "Use h to move left."<CR>
map <Right> :echo "Use l to move right."<CR>
