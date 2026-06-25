abort("Expected Ruby 3.4+, but got #{RUBY_VERSION}.") if RUBY_VERSION < '3.4.0'

require 'lvr'

Package = Data.define(:name) do
  def path = "lib/#{self.name}"
  def library = { name: self.name.to_s.gsub('-', '_') }
  def binaries = []
  def label = self.metadata[:label] || self.name
  def summary
    manifest = self.manifest
    !(self.metadata.empty?) ?
      self.metadata[:summary] :
      manifest.package[:description].then { it.is_a?(String) ? it : nil }
  end
  def import = self.examples[:import] || "#{self.library[:name]}::*"
  def examples = self.metadata[:examples] || {}
  def features = self.metadata[:features] || []
  def metadata = self.manifest.package[:metadata][:lvr] rescue {}
  def manifest = Lvr::Rust::Manifest.load("#{self.path}/Cargo.toml")
  def to_liquid = self.to_h
  def to_h = { name:, path:, library:, binaries:, label:, summary:, import:, examples:, features:, metadata:, manifest: self.manifest.to_h }
end

def codegen(**context) = ->(t, _) { Lvr.codegen(t.name, t.source, **CONTEXT.merge(context)) }
def copy = ->(t, _) { cp t.source, t.name }

CRATES = Dir['lib/*'].map { |p| p.delete_prefix!("lib/") }.sort!.freeze
READER_CRATES = CRATES.grep(/^rdf-reader-/).freeze
WRITER_CRATES = CRATES.grep(/^rdf-writer-/).freeze
STORE_CRATES  = CRATES.grep(/^rdf-store-/).freeze
CORE_CRATES   = CRATES - READER_CRATES - WRITER_CRATES - STORE_CRATES - %w[rdf_rs rdf-borsh]

task :default => %w[README.md] +
  READER_CRATES.map { |c| "lib/#{c}/README.md" } +
  WRITER_CRATES.map { |c| "lib/#{c}/README.md" }
  #STORE_CRATES.map { |c| "lib/#{c}/README.md" } # TODO
file 'README.md' => %w[.config/codegen/README.md.liquid], &codegen

CRATES.each do |crate_name|
  package = Package.new(crate_name)
  crate_codegen = codegen(package: package.to_h)
  case crate_name
    when "rdf_rs"
    when /^rdf-reader-/
      file "lib/#{crate_name}/README.md" => %w[.config/codegen/rdf-reader/README.md.liquid], &crate_codegen
    when /^rdf-writer-/
      file "lib/#{crate_name}/README.md" => %w[.config/codegen/rdf-writer/README.md.liquid], &crate_codegen
    when /^rdf-store-/
      file "lib/#{crate_name}/README.md" => %w[.config/codegen/rdf-store/README.md.liquid], &crate_codegen
  end
end

CONTEXT = {
  project: {
    title: "RDF.rs",
  },
  github: {
    repository: {
      link: 'https://github.com/rust-rdf/rdf.rs',
      url:  'https://github.com/rust-rdf/rdf.rs.git',
    }
  },
  packages: {
    core: CORE_CRATES.map { Package.new(it).to_h },
    reader: READER_CRATES.map { Package.new(it).to_h },
    writer: WRITER_CRATES.map { Package.new(it).to_h },
    store: STORE_CRATES.map { Package.new(it).to_h },
  }
}
