module.exports = {
  cors: true,
  open: false,
  ui: false,
  proxy: `http://${process.env.HOST}:${process.env.PORT}`,
  host: "0.0.0.0",
  port: "8000",
  files: ["target/debug/sysmet-http"],
  socket: {
    domain: "https://8000.example.org",
  },
};

