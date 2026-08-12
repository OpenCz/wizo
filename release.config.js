module.exports = {
  branches: [
    "stable",
    { name: 'staging', prerelease: 'alpha' }
  ],
  plugins: [
    ['@semantic-release/commit-analyzer', {
      preset: 'conventionalcommits',
      releaseRules: [
        { type: 'feat!', release: 'major' },
        { type: 'fix!', release: 'major' }
      ]
    }],

    ['@semantic-release/release-notes-generator', {
      preset: 'conventionalcommits',
      presetConfig: {
        types: [
          { type: 'feat', section: 'New APIs / Features', hidden: false },
          { type: 'fix', section: 'Bug Fixes', hidden: false },
          { type: 'feat!', section: 'Breaking Changes / Features', hidden: false },
          { type: 'fix!', section: 'Breaking Changes / Bug Fixes', hidden: false },
          { type: 'chore', hidden: true }
        ]
      }
    }],

    ["@semantic-release-cargo/semantic-release-cargo"],

    ['@semantic-release/git', {
      assets: ['Cargo.toml', 'Cargo.lock'],
      message: 'chore(release): v${nextRelease.version} [skip ci]'
    }],

    '@semantic-release/github'
  ]
};
