SHADERS_DIR = shaders/
SHADERS_FILE = shader.frag shader.vert
SHADERS_SPV = $(addprefix $(SHADERS_DIR), $(addsuffix .spv, $(SHADERS_FILE)))
SHADERS = $(addprefix $(SHADERS_DIR), $(SHADERS_FILE))

all: $(SHADERS_SPV)
	cargo run

release: $(SHADERS_SPV)
	cargo run --release

shaders: $(SHADERS_SPV)

$(SHADERS_DIR)%.spv: $(SHADERS_DIR)%
	glslangValidator -V $< -o $@

clean:
	cargo clean

fclean: clean
	rm -f $(SHADERS_SPV)

re: fclean all

.PHONY: all release clean fclean re shaders
