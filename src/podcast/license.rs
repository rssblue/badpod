use serde::{Deserialize, Deserializer};
use std::fmt;
use strum::{EnumProperty, IntoEnumIterator};
use strum_macros::{EnumIter, EnumProperty};

/// Type of [License](crate::podcast::License).
#[derive(Debug, PartialEq, Eq, EnumProperty, EnumIter)]
pub enum LicenseType {
    #[strum(props(str = "0bsd"))]
    BsdZeroClause,
    #[strum(props(str = "aal"))]
    AttributionAssurance,
    #[strum(props(str = "abstyles"))]
    Abstyles,
    #[strum(props(str = "adobe-2006"))]
    AdobeSystemsIncorporatedSourceCodeAgreement,
    #[strum(props(str = "adobe-glyph"))]
    AdobeGlyphList,
    #[strum(props(str = "adsl"))]
    AmazonDigitalServices,
    #[strum(props(str = "afl-1.1"))]
    AcademicFreeV1_1,
    #[strum(props(str = "afl-1.2"))]
    AcademicFreeV1_2,
    #[strum(props(str = "afl-2.0"))]
    AcademicFreeV2_0,
    #[strum(props(str = "afl-2.1"))]
    AcademicFreeV2_1,
    #[strum(props(str = "afl-3.0"))]
    AcademicFreeV3_0,
    #[strum(props(str = "afmparse"))]
    Afmparse,
    #[strum(props(str = "agpl-1.0-only"))]
    AfferoGeneralPublicV1_0only,
    #[strum(props(str = "agpl-1.0-or-later"))]
    AfferoGeneralPublicV1_0OrLater,
    #[strum(props(str = "agpl-3.0-only"))]
    GnuAfferoGeneralPublicV3_0only,
    #[strum(props(str = "agpl-3.0-or-later"))]
    GnuAfferoGeneralPublicV3_0OrLater,
    #[strum(props(str = "aladdin"))]
    AladdinFreePublic,
    #[strum(props(str = "amdplpa"))]
    AmdPlpaMapC,
    #[strum(props(str = "aml"))]
    AppleMit,
    #[strum(props(str = "ampas"))]
    AcademyofMotionPictureArtsAndSciencesBsd,
    #[strum(props(str = "antlr-pd"))]
    AntlrSoftwareRightsNotice,
    #[strum(props(str = "antlr-pd-fallback"))]
    AntlrSoftwareRightsNoticewithlicensefallback,
    #[strum(props(str = "apache-1.0"))]
    Apache1_0,
    #[strum(props(str = "apache-1.1"))]
    Apache1_1,
    #[strum(props(str = "apache-2.0"))]
    Apache2_0,
    #[strum(props(str = "apafml"))]
    AdobePostscriptAfm,
    #[strum(props(str = "apl-1.0"))]
    AdaptivePublic1_0,
    #[strum(props(str = "app-s2p"))]
    AppS2p,
    #[strum(props(str = "apsl-1.0"))]
    ApplePublicSource1_0,
    #[strum(props(str = "apsl-1.1"))]
    ApplePublicSource1_1,
    #[strum(props(str = "apsl-1.2"))]
    ApplePublicSource1_2,
    #[strum(props(str = "apsl-2.0"))]
    ApplePublicSource2_0,
    #[strum(props(str = "arphic-1999"))]
    ArphicPublic,
    #[strum(props(str = "artistic-1.0"))]
    Artistic1_0,
    #[strum(props(str = "artistic-1.0-cl8"))]
    Artistic1_0wClause8,
    #[strum(props(str = "artistic-1.0-perl"))]
    Artistic1_0Perl,
    #[strum(props(str = "artistic-2.0"))]
    Artistic2_0,
    #[strum(props(str = "baekmuk"))]
    Baekmuk,
    #[strum(props(str = "bahyph"))]
    Bahyph,
    #[strum(props(str = "barr"))]
    Barr,
    #[strum(props(str = "beerware"))]
    Beerware,
    #[strum(props(str = "bitstream-vera"))]
    BitstreamVeraFont,
    #[strum(props(str = "bittorrent-1.0"))]
    BitTorrentOpenSourceV1_0,
    #[strum(props(str = "bittorrent-1.1"))]
    BitTorrentOpenSourceV1_1,
    #[strum(props(str = "blessing"))]
    SqliteBlessing,
    #[strum(props(str = "blueoak-1.0.0"))]
    BlueOakModel1_0_0,
    #[strum(props(str = "borceux"))]
    Borceuxlicense,
    #[strum(props(str = "bsd-1-clause"))]
    Bsd1Clause,
    #[strum(props(str = "bsd-2-clause"))]
    Bsd2ClauseSimplified,
    #[strum(props(str = "bsd-2-clause-patent"))]
    Bsd2ClausePlusPatent,
    #[strum(props(str = "bsd-2-clause-views"))]
    Bsd2Clausewithviewssentence,
    #[strum(props(str = "bsd-3-clause"))]
    Bsd3ClauseNewOrRevised,
    #[strum(props(str = "bsd-3-clause-attribution"))]
    BsdWithAttribution,
    #[strum(props(str = "bsd-3-clause-clear"))]
    Bsd3ClauseClear,
    #[strum(props(str = "bsd-3-clause-lbnl"))]
    LawrenceBerkeleyNationalLabsBsdVariantlicense,
    #[strum(props(str = "bsd-3-clause-modification"))]
    Bsd3ClauseModification,
    #[strum(props(str = "bsd-3-clause-no-military-license"))]
    Bsd3ClauseNoMilitary,
    #[strum(props(str = "bsd-3-clause-no-nuclear-license"))]
    Bsd3ClauseNoNuclear,
    #[strum(props(str = "bsd-3-clause-no-nuclear-license-2014"))]
    Bsd3ClauseNoNuclear2014,
    #[strum(props(str = "bsd-3-clause-no-nuclear-warranty"))]
    Bsd3ClauseNoNuclearWarranty,
    #[strum(props(str = "bsd-3-clause-open-mpi"))]
    Bsd3ClauseOpenMpiVariant,
    #[strum(props(str = "bsd-4-clause"))]
    Bsd4ClauseOriginalOrOld,
    #[strum(props(str = "bsd-4-clause-shortened"))]
    Bsd4ClauseShortened,
    #[strum(props(str = "bsd-4-clause-uc"))]
    Bsd4ClauseUniversityofCaliforniaSpecific,
    #[strum(props(str = "bsd-protection"))]
    BsdProtection,
    #[strum(props(str = "bsd-source-code"))]
    BsdSourceCodeAttribution,
    #[strum(props(str = "bsl-1.0"))]
    BoostSoftware1_0,
    #[strum(props(str = "busl-1.1"))]
    BusinessSource1_1,
    #[strum(props(str = "bzip2-1.0.6"))]
    Bzip2AndLibbzip2LicenseV1_0_6,
    #[strum(props(str = "c-uda-1.0"))]
    ComputationalUseofDataAgreementV1_0,
    #[strum(props(str = "cal-1.0"))]
    CryptographicAutonomy1_0,
    #[strum(props(str = "cal-1.0-combined-work-exception"))]
    CryptographicAutonomy1_0CombinedWorkException,
    #[strum(props(str = "caldera"))]
    Caldera,
    #[strum(props(str = "catosl-1.1"))]
    ComputerAssociatesTrustedOpenSource1_1,
    #[strum(props(str = "cc-by-1.0"))]
    CreativeCommonsAttribution1_0Generic,
    #[strum(props(str = "cc-by-2.0"))]
    CreativeCommonsAttribution2_0Generic,
    #[strum(props(str = "cc-by-2.5"))]
    CreativeCommonsAttribution2_5Generic,
    #[strum(props(str = "cc-by-2.5-au"))]
    CreativeCommonsAttribution2_5Australia,
    #[strum(props(str = "cc-by-3.0"))]
    CreativeCommonsAttribution3_0Unported,
    #[strum(props(str = "cc-by-3.0-at"))]
    CreativeCommonsAttribution3_0Austria,
    #[strum(props(str = "cc-by-3.0-de"))]
    CreativeCommonsAttribution3_0Germany,
    #[strum(props(str = "cc-by-3.0-igo"))]
    CreativeCommonsAttribution3_0Igo,
    #[strum(props(str = "cc-by-3.0-nl"))]
    CreativeCommonsAttribution3_0Netherlands,
    #[strum(props(str = "cc-by-3.0-us"))]
    CreativeCommonsAttribution3_0UnitedStates,
    #[strum(props(str = "cc-by-4.0"))]
    CreativeCommonsAttribution4_0International,
    #[strum(props(str = "cc-by-nc-1.0"))]
    CreativeCommonsAttributionNonCommercial1_0Generic,
    #[strum(props(str = "cc-by-nc-2.0"))]
    CreativeCommonsAttributionNonCommercial2_0Generic,
    #[strum(props(str = "cc-by-nc-2.5"))]
    CreativeCommonsAttributionNonCommercial2_5Generic,
    #[strum(props(str = "cc-by-nc-3.0"))]
    CreativeCommonsAttributionNonCommercial3_0Unported,
    #[strum(props(str = "cc-by-nc-3.0-de"))]
    CreativeCommonsAttributionNonCommercial3_0Germany,
    #[strum(props(str = "cc-by-nc-4.0"))]
    CreativeCommonsAttributionNonCommercial4_0International,
    #[strum(props(str = "cc-by-nc-nd-1.0"))]
    CreativeCommonsAttributionNonCommercialNoDerivatives1_0Generic,
    #[strum(props(str = "cc-by-nc-nd-2.0"))]
    CreativeCommonsAttributionNonCommercialNoDerivatives2_0Generic,
    #[strum(props(str = "cc-by-nc-nd-2.5"))]
    CreativeCommonsAttributionNonCommercialNoDerivatives2_5Generic,
    #[strum(props(str = "cc-by-nc-nd-3.0"))]
    CreativeCommonsAttributionNonCommercialNoDerivatives3_0Unported,
    #[strum(props(str = "cc-by-nc-nd-3.0-de"))]
    CreativeCommonsAttributionNonCommercialNoDerivatives3_0Germany,
    #[strum(props(str = "cc-by-nc-nd-3.0-igo"))]
    CreativeCommonsAttributionNonCommercialNoDerivatives3_0Igo,
    #[strum(props(str = "cc-by-nc-nd-4.0"))]
    CreativeCommonsAttributionNonCommercialNoDerivatives4_0International,
    #[strum(props(str = "cc-by-nc-sa-1.0"))]
    CreativeCommonsAttributionNonCommercialShareAlike1_0Generic,
    #[strum(props(str = "cc-by-nc-sa-2.0"))]
    CreativeCommonsAttributionNonCommercialShareAlike2_0Generic,
    #[strum(props(str = "cc-by-nc-sa-2.0-fr"))]
    CreativeCommonsAttributionNonCommercialShareAlike2_0France,
    #[strum(props(str = "cc-by-nc-sa-2.0-uk"))]
    CreativeCommonsAttributionNonCommercialShareAlike2_0EnglandandWales,
    #[strum(props(str = "cc-by-nc-sa-2.5"))]
    CreativeCommonsAttributionNonCommercialShareAlike2_5Generic,
    #[strum(props(str = "cc-by-nc-sa-3.0"))]
    CreativeCommonsAttributionNonCommercialShareAlike3_0Unported,
    #[strum(props(str = "cc-by-nc-sa-3.0-de"))]
    CreativeCommonsAttributionNonCommercialShareAlike3_0Germany,
    #[strum(props(str = "cc-by-nc-sa-3.0-igo"))]
    CreativeCommonsAttributionNonCommercialShareAlike3_0Igo,
    #[strum(props(str = "cc-by-nc-sa-4.0"))]
    CreativeCommonsAttributionNonCommercialShareAlike4_0International,
    #[strum(props(str = "cc-by-nd-1.0"))]
    CreativeCommonsAttributionNoDerivatives1_0Generic,
    #[strum(props(str = "cc-by-nd-2.0"))]
    CreativeCommonsAttributionNoDerivatives2_0Generic,
    #[strum(props(str = "cc-by-nd-2.5"))]
    CreativeCommonsAttributionNoDerivatives2_5Generic,
    #[strum(props(str = "cc-by-nd-3.0"))]
    CreativeCommonsAttributionNoDerivatives3_0Unported,
    #[strum(props(str = "cc-by-nd-3.0-de"))]
    CreativeCommonsAttributionNoDerivatives3_0Germany,
    #[strum(props(str = "cc-by-nd-4.0"))]
    CreativeCommonsAttributionNoDerivatives4_0International,
    #[strum(props(str = "cc-by-sa-1.0"))]
    CreativeCommonsAttributionShareAlike1_0Generic,
    #[strum(props(str = "cc-by-sa-2.0"))]
    CreativeCommonsAttributionShareAlike2_0Generic,
    #[strum(props(str = "cc-by-sa-2.0-uk"))]
    CreativeCommonsAttributionShareAlike2_0EnglAndandWales,
    #[strum(props(str = "cc-by-sa-2.1-jp"))]
    CreativeCommonsAttributionShareAlike2_1Japan,
    #[strum(props(str = "cc-by-sa-2.5"))]
    CreativeCommonsAttributionShareAlike2_5Generic,
    #[strum(props(str = "cc-by-sa-3.0"))]
    CreativeCommonsAttributionShareAlike3_0Unported,
    #[strum(props(str = "cc-by-sa-3.0-at"))]
    CreativeCommonsAttributionShareAlike3_0Austria,
    #[strum(props(str = "cc-by-sa-3.0-de"))]
    CreativeCommonsAttributionShareAlike3_0Germany,
    #[strum(props(str = "cc-by-sa-4.0"))]
    CreativeCommonsAttributionShareAlike4_0International,
    #[strum(props(str = "cc-pddc"))]
    CreativeCommonsPublicDomainDedicationAndCertification,
    #[strum(props(str = "cc0-1.0"))]
    CreativeCommonsZeroV1_0Universal,
    #[strum(props(str = "cddl-1.0"))]
    CommonDevelopmentAndDistribution1_0,
    #[strum(props(str = "cddl-1.1"))]
    CommonDevelopmentAndDistribution1_1,
    #[strum(props(str = "cdl-1.0"))]
    CommonDocumentation1_0,
    #[strum(props(str = "cdla-permissive-1.0"))]
    CommunityDataAgreementPermissive1_0,
    #[strum(props(str = "cdla-permissive-2.0"))]
    CommunityDataAgreementPermissive2_0,
    #[strum(props(str = "cdla-sharing-1.0"))]
    CommunityDataAgreementSharing1_0,
    #[strum(props(str = "cecill-1.0"))]
    CeCillFreeSoftwareAgreementV1_0,
    #[strum(props(str = "cecill-1.1"))]
    CeCillFreeSoftwareAgreementV1_1,
    #[strum(props(str = "cecill-2.0"))]
    CeCillFreeSoftwareAgreementV2_0,
    #[strum(props(str = "cecill-2.1"))]
    CeCillFreeSoftwareAgreementV2_1,
    #[strum(props(str = "cecill-b"))]
    CeCillBFreeSoftwareAgreement,
    #[strum(props(str = "cecill-c"))]
    CeCillCFreeSoftwareAgreement,
    #[strum(props(str = "cern-ohl-1.1"))]
    CernOpenHardwareLicenceV1_1,
    #[strum(props(str = "cern-ohl-1.2"))]
    CernOpenHardwareLicenceV1_2,
    #[strum(props(str = "cern-ohl-p-2.0"))]
    CernOpenHardwareLicenceVersion2Permissive,
    #[strum(props(str = "cern-ohl-s-2.0"))]
    CernOpenHardwareLicenceVersion2StronglyReciprocal,
    #[strum(props(str = "cern-ohl-w-2.0"))]
    CernOpenHardwareLicenceVersion2WeaklyReciprocal,
    #[strum(props(str = "clartistic"))]
    ClarifiedArtistic,
    #[strum(props(str = "cnri-jython"))]
    CnriJython,
    #[strum(props(str = "cnri-python"))]
    CnriPython,
    #[strum(props(str = "cnri-python-gpl-compatible"))]
    CnriPythonOpenSourceGplCompatibleAgreement,
    #[strum(props(str = "coil-1.0"))]
    CopyfreeOpenInnovation,
    #[strum(props(str = "community-spec-1.0"))]
    CommunitySpecification1_0,
    #[strum(props(str = "condor-1.1"))]
    CondorPublicV1_1,
    #[strum(props(str = "copyleft-next-0.3.0"))]
    Copyleftnext0_3_0,
    #[strum(props(str = "copyleft-next-0.3.1"))]
    Copyleftnext0_3_1,
    #[strum(props(str = "cpal-1.0"))]
    CommonPublicAttribution1_0,
    #[strum(props(str = "cpl-1.0"))]
    CommonPublic1_0,
    #[strum(props(str = "cpol-1.02"))]
    CodeProjectOpen1_02,
    #[strum(props(str = "crossword"))]
    Crossword,
    #[strum(props(str = "crystalstacker"))]
    CrystalStacker,
    #[strum(props(str = "cua-opl-1.0"))]
    CuaOfficePublicV1_0,
    #[strum(props(str = "cube"))]
    Cube,
    #[strum(props(str = "curl"))]
    Curl,
    #[strum(props(str = "d-fsl-1.0"))]
    DeutscheFreieSoftwareLizenz,
    #[strum(props(str = "diffmark"))]
    Diffmark,
    #[strum(props(str = "dl-de-by-2.0"))]
    DatalicenceGermanyAttributionVersion2_0,
    #[strum(props(str = "doc"))]
    Doc,
    #[strum(props(str = "dotseqn"))]
    Dotseqn,
    #[strum(props(str = "drl-1.0"))]
    DetectionRule1_0,
    #[strum(props(str = "dsdp"))]
    Dsdp,
    #[strum(props(str = "dvipdfm"))]
    Dvipdfm,
    #[strum(props(str = "ecl-1.0"))]
    EducationalCommunityV1_0,
    #[strum(props(str = "ecl-2.0"))]
    EducationalCommunityV2_0,
    #[strum(props(str = "efl-1.0"))]
    EiffelForumV1_0,
    #[strum(props(str = "efl-2.0"))]
    EiffelForumV2_0,
    #[strum(props(str = "egenix"))]
    EGenixComPublic1_1_0,
    #[strum(props(str = "elastic-2.0"))]
    Elastic2_0,
    #[strum(props(str = "entessa"))]
    EntessaPublicV1_0,
    #[strum(props(str = "epics"))]
    EpicsOpen,
    #[strum(props(str = "epl-1.0"))]
    EclipsePublic1_0,
    #[strum(props(str = "epl-2.0"))]
    EclipsePublic2_0,
    #[strum(props(str = "erlpl-1.1"))]
    ErlangPublicV1_1,
    #[strum(props(str = "etalab-2.0"))]
    EtalabOpen2_0,
    #[strum(props(str = "eudatagrid"))]
    EuDataGridSoftware,
    #[strum(props(str = "eupl-1.0"))]
    EuropeanUnionPublic1_0,
    #[strum(props(str = "eupl-1.1"))]
    EuropeanUnionPublic1_1,
    #[strum(props(str = "eupl-1.2"))]
    EuropeanUnionPublic1_2,
    #[strum(props(str = "eurosym"))]
    Eurosym,
    #[strum(props(str = "fair"))]
    Fair,
    #[strum(props(str = "fdk-aac"))]
    FraunhoferFdkAacCodecLibrary,
    #[strum(props(str = "frameworx-1.0"))]
    FrameworxOpen1_0,
    #[strum(props(str = "freebsd-doc"))]
    FreeBsdDocumentation,
    #[strum(props(str = "freeimage"))]
    FreeImagePublicV1_0,
    #[strum(props(str = "fsfap"))]
    FsfAllPermissive,
    #[strum(props(str = "fsful"))]
    FsfUnlimited,
    #[strum(props(str = "fsfullr"))]
    FsfUnlimitedWithLicenseRetention,
    #[strum(props(str = "ftl"))]
    FreetypeProject,
    #[strum(props(str = "gd"))]
    Gd,
    #[strum(props(str = "gfdl-1.1-invariants-only"))]
    GnuFreeDocumentationV1_1OnlyInvariants,
    #[strum(props(str = "gfdl-1.1-invariants-or-later"))]
    GnuFreeDocumentationV1_1OrLaterInvariants,
    #[strum(props(str = "gfdl-1.1-no-invariants-only"))]
    GnuFreeDocumentationV1_1OnlyNoInvariants,
    #[strum(props(str = "gfdl-1.1-no-invariants-or-later"))]
    GnuFreeDocumentationV1_1OrLaterNoInvariants,
    #[strum(props(str = "gfdl-1.1-only"))]
    GnuFreeDocumentationV1_1Only,
    #[strum(props(str = "gfdl-1.1-or-later"))]
    GnuFreeDocumentationV1_1OrLater,
    #[strum(props(str = "gfdl-1.2-invariants-only"))]
    GnuFreeDocumentationV1_2OnlyInvariants,
    #[strum(props(str = "gfdl-1.2-invariants-or-later"))]
    GnuFreeDocumentationV1_2OrLaterInvariants,
    #[strum(props(str = "gfdl-1.2-no-invariants-only"))]
    GnuFreeDocumentationV1_2OnlyNoInvariants,
    #[strum(props(str = "gfdl-1.2-no-invariants-or-later"))]
    GnuFreeDocumentationV1_2OrLaterNoInvariants,
    #[strum(props(str = "gfdl-1.2-only"))]
    GnuFreeDocumentationV1_2Only,
    #[strum(props(str = "gfdl-1.2-or-later"))]
    GnuFreeDocumentationV1_2OrLater,
    #[strum(props(str = "gfdl-1.3-invariants-only"))]
    GnuFreeDocumentationV1_3OnlyInvariants,
    #[strum(props(str = "gfdl-1.3-invariants-or-later"))]
    GnuFreeDocumentationV1_3OrLaterInvariants,
    #[strum(props(str = "gfdl-1.3-no-invariants-only"))]
    GnuFreeDocumentationV1_3OnlyNoInvariants,
    #[strum(props(str = "gfdl-1.3-no-invariants-or-later"))]
    GnuFreeDocumentationV1_3OrLaterNoInvariants,
    #[strum(props(str = "gfdl-1.3-only"))]
    GnuFreeDocumentationV1_3Only,
    #[strum(props(str = "gfdl-1.3-or-later"))]
    GnuFreeDocumentationV1_3OrLater,
    #[strum(props(str = "giftware"))]
    Giftware,
    #[strum(props(str = "gl2ps"))]
    Gl2ps,
    #[strum(props(str = "glide"))]
    DfxGlide,
    #[strum(props(str = "glulxe"))]
    Glulxe,
    #[strum(props(str = "glwtpl"))]
    GoodLuckWithThatPublic,
    #[strum(props(str = "gnuplot"))]
    GnuPlot,
    #[strum(props(str = "gpl-1.0-only"))]
    GnuGeneralPublicV1_0only,
    #[strum(props(str = "gpl-1.0-or-later"))]
    GnuGeneralPublicV1_0OrLater,
    #[strum(props(str = "gpl-2.0-only"))]
    GnuGeneralPublicV2_0only,
    #[strum(props(str = "gpl-2.0-or-later"))]
    GnuGeneralPublicV2_0OrLater,
    #[strum(props(str = "gpl-3.0-only"))]
    GnuGeneralPublicV3_0only,
    #[strum(props(str = "gpl-3.0-or-later"))]
    GnuGeneralPublicV3_0OrLater,
    #[strum(props(str = "gsoap-1.3b"))]
    GsoapPublicV1_3b,
    #[strum(props(str = "haskellreport"))]
    HaskellLanguageReport,
    #[strum(props(str = "hippocratic-2.1"))]
    Hippocratic2_1,
    #[strum(props(str = "hpnd"))]
    HistoricalPermissionNoticeAndDisclaimer,
    #[strum(props(str = "hpnd-sell-variant"))]
    HistoricalPermissionNoticeAndDisclaimersellvariant,
    #[strum(props(str = "htmltidy"))]
    HtmlTidy,
    #[strum(props(str = "ibm-pibs"))]
    IbmPowerPcInitializationAndBootSoftware,
    #[strum(props(str = "icu"))]
    Icu,
    #[strum(props(str = "ijg"))]
    IndependentJpegGroup,
    #[strum(props(str = "imagemagick"))]
    ImageMagick,
    #[strum(props(str = "imatix"))]
    IMatixStAndardFunctionLibraryAgreement,
    #[strum(props(str = "imlib2"))]
    Imlib2License,
    #[strum(props(str = "info-zip"))]
    InfoZip,
    #[strum(props(str = "intel"))]
    IntelOpenSource,
    #[strum(props(str = "intel-acpi"))]
    IntelAcpiSoftwareAgreement,
    #[strum(props(str = "interbase-1.0"))]
    InterbasePublicV1_0,
    #[strum(props(str = "ipa"))]
    IpaFont,
    #[strum(props(str = "ipl-1.0"))]
    IbmPublicV1_0,
    #[strum(props(str = "isc"))]
    Isc,
    #[strum(props(str = "jam"))]
    Jam,
    #[strum(props(str = "jasper-2.0"))]
    JasPer,
    #[strum(props(str = "jpnic"))]
    JapanNetworkInformationCenter,
    #[strum(props(str = "json"))]
    Json,
    #[strum(props(str = "lal-1.2"))]
    LicenceArtLibre1_2,
    #[strum(props(str = "lal-1.3"))]
    LicenceArtLibre1_3,
    #[strum(props(str = "latex2e"))]
    Latex2e,
    #[strum(props(str = "leptonica"))]
    Leptonica,
    #[strum(props(str = "lgpl-2.0-only"))]
    GnuLibraryGeneralPublicV2only,
    #[strum(props(str = "lgpl-2.0-or-later"))]
    GnuLibraryGeneralPublicV2OrLater,
    #[strum(props(str = "lgpl-2.1-only"))]
    GnuLesserGeneralPublicV2_1only,
    #[strum(props(str = "lgpl-2.1-or-later"))]
    GnuLesserGeneralPublicV2_1OrLater,
    #[strum(props(str = "lgpl-3.0-only"))]
    GnuLesserGeneralPublicV3_0Only,
    #[strum(props(str = "lgpl-3.0-or-later"))]
    GnuLesserGeneralPublicV3_0OrLater,
    #[strum(props(str = "lgpllr"))]
    LesserGeneralPublicForLinguisticResources,
    #[strum(props(str = "libpng"))]
    Libpng,
    #[strum(props(str = "libpng-2.0"))]
    PngReferenceLibraryversion2,
    #[strum(props(str = "libselinux-1.0"))]
    LibselinuxPublicDomaiNnotice,
    #[strum(props(str = "libtiff"))]
    Libtiff,
    #[strum(props(str = "liliq-p-1.1"))]
    LicenceLibreduQuebecPermissiveversion1_1,
    #[strum(props(str = "liliq-r-1.1"))]
    LicenceLibreduQuebecReciprociteversion1_1,
    #[strum(props(str = "liliq-rplus-1.1"))]
    LicenceLibreduQuebecReciprociteforteversion1_1,
    #[strum(props(str = "linux-man-pages-copyleft"))]
    LinuxmanpagesCopyleft,
    #[strum(props(str = "linux-openib"))]
    LinuxKernelVariantofOpenIbOrg,
    #[strum(props(str = "lpl-1.0"))]
    LucentPublicVersion1_0,
    #[strum(props(str = "lpl-1.02"))]
    LucentPublicV1_02,
    #[strum(props(str = "lppl-1.0"))]
    LaTeXProjectPublicV1_0,
    #[strum(props(str = "lppl-1.1"))]
    LaTeXProjectPublicV1_1,
    #[strum(props(str = "lppl-1.2"))]
    LaTeXProjectPublicV1_2,
    #[strum(props(str = "lppl-1.3a"))]
    LaTeXProjectPublicV1_3a,
    #[strum(props(str = "lppl-1.3c"))]
    LaTeXProjectPublicV1_3c,
    #[strum(props(str = "lzma-sdk-9.11-to-9.20"))]
    LzmaSdkVersions9_11To9_20,
    #[strum(props(str = "lzma-sdk-9.22"))]
    LzmaSdkVersions9_22AndBeyond,
    #[strum(props(str = "makeindex"))]
    MakeIndex,
    #[strum(props(str = "minpack"))]
    Minpack,
    #[strum(props(str = "miros"))]
    TheMirOsLicence,
    #[strum(props(str = "mit"))]
    Mit,
    #[strum(props(str = "mit-0"))]
    MitNoAttribution,
    #[strum(props(str = "mit-advertising"))]
    EnlightenmentE16,
    #[strum(props(str = "mit-cmu"))]
    Cmu,
    #[strum(props(str = "mit-enna"))]
    Enna,
    #[strum(props(str = "mit-feh"))]
    Feh,
    #[strum(props(str = "mit-modern-variant"))]
    MitModernVariant,
    #[strum(props(str = "mit-open-group"))]
    MitOpenGroupvariant,
    #[strum(props(str = "mitnfa"))]
    MitNoFalseAttribs,
    #[strum(props(str = "motosoto"))]
    Motosoto,
    #[strum(props(str = "mpi-permissive"))]
    MpiPermissive,
    #[strum(props(str = "mpich2"))]
    Mpich2License,
    #[strum(props(str = "mpl-1.0"))]
    MozillaPublic1_0,
    #[strum(props(str = "mpl-1.1"))]
    MozillaPublic1_1,
    #[strum(props(str = "mpl-2.0"))]
    MozillaPublic2_0,
    #[strum(props(str = "mpl-2.0-no-copyleft-exception"))]
    MozillaPublic2_0nocopyleftexception,
    #[strum(props(str = "mplus"))]
    MplusFont,
    #[strum(props(str = "ms-lpl"))]
    MicrosoftLimitedPublic,
    #[strum(props(str = "ms-pl"))]
    MicrosoftPublic,
    #[strum(props(str = "ms-rl"))]
    MicrosoftReciprocal,
    #[strum(props(str = "mtll"))]
    MatrixTemplateLibrary,
    #[strum(props(str = "mulanpsl-1.0"))]
    MulanPermissiveSoftwareVersion1,
    #[strum(props(str = "mulanpsl-2.0"))]
    MulanPermissiveSoftwareVersion2,
    #[strum(props(str = "multics"))]
    Multics,
    #[strum(props(str = "mup"))]
    Mup,
    #[strum(props(str = "naist-2003"))]
    NaraInstituteofScienceAndTechnology2003,
    #[strum(props(str = "nasa-1.3"))]
    NasaOpenSourceAgreement1_3,
    #[strum(props(str = "naumen"))]
    NaumenPublic,
    #[strum(props(str = "nbpl-1.0"))]
    NetBooleanPublicV1,
    #[strum(props(str = "ncgl-uk-2.0"))]
    NonCommercialGovernmentLicence,
    #[strum(props(str = "ncsa"))]
    UniversityofIllinoisNcsaOpenSource,
    #[strum(props(str = "net-snmp"))]
    NetSnmp,
    #[strum(props(str = "netcdf"))]
    NetCdf,
    #[strum(props(str = "newsletr"))]
    Newsletr,
    #[strum(props(str = "ngpl"))]
    NethackGeneralPublic,
    #[strum(props(str = "nicta-1.0"))]
    NictaPublicSoftware,
    Version1_0,
    #[strum(props(str = "nist-pd"))]
    NistPublicDomainNotice,
    #[strum(props(str = "nist-pd-fallback"))]
    NistPublicDomainNoticewithlicensefallback,
    #[strum(props(str = "nlod-1.0"))]
    NorwegianLicenceforOpenGovernmentDataNlod1_0,
    #[strum(props(str = "nlod-2.0"))]
    NorwegianLicenceforOpenGovernmentDataNlod2_0,
    #[strum(props(str = "nlpl"))]
    NoLimitPublic,
    #[strum(props(str = "nokia"))]
    NokiaOpenSource,
    #[strum(props(str = "nosl"))]
    NetizenOpenSource,
    #[strum(props(str = "noweb"))]
    Noweb,
    #[strum(props(str = "npl-1.0"))]
    NetscapePublicV1_0,
    #[strum(props(str = "npl-1.1"))]
    NetscapePublicV1_1,
    #[strum(props(str = "nposl-3.0"))]
    NonProfitOpenSoftware3_0,
    #[strum(props(str = "nrl"))]
    Nrl,
    #[strum(props(str = "ntp"))]
    Ntp,
    #[strum(props(str = "ntp-0"))]
    NtpNoAttribution,
    #[strum(props(str = "o-uda-1.0"))]
    OpenUseofDataAgreementV1_0,
    #[strum(props(str = "occt-pl"))]
    OpenCascadeTechnologyPublic,
    #[strum(props(str = "oclc-2.0"))]
    OclcResearchPublic2_0,
    #[strum(props(str = "odbl-1.0"))]
    OpenDataCommonsOpenDatabaseV1_0,
    #[strum(props(str = "odc-by-1.0"))]
    OpenDataCommonsAttributionV1_0,
    #[strum(props(str = "ofl-1.0"))]
    SilOpenFont1_0,
    #[strum(props(str = "ofl-1.0-no-rfn"))]
    SilOpenFont1_0withnoReservedFontName,
    #[strum(props(str = "ofl-1.0-rfn"))]
    SilOpenFont1_0withReservedFontName,
    #[strum(props(str = "ofl-1.1"))]
    SilOpenFont1_1,
    #[strum(props(str = "ofl-1.1-no-rfn"))]
    SilOpenFont1_1withnoReservedFontName,
    #[strum(props(str = "ofl-1.1-rfn"))]
    SilOpenFont1_1withReservedFontName,
    #[strum(props(str = "ogc-1.0"))]
    OgcSoftwareVersion1_0,
    #[strum(props(str = "ogdl-taiwan-1.0"))]
    TaiwanOpenGovernmentDataVersion1_0,
    #[strum(props(str = "ogl-canada-2.0"))]
    OpenGovernmentLicenceCanada,
    #[strum(props(str = "ogl-uk-1.0"))]
    OpenGovernmentLicenceV1_0,
    #[strum(props(str = "ogl-uk-2.0"))]
    OpenGovernmentLicenceV2_0,
    #[strum(props(str = "ogl-uk-3.0"))]
    OpenGovernmentLicenceV3_0,
    #[strum(props(str = "ogtsl"))]
    OpenGroupTestSuite,
    #[strum(props(str = "oldap-1.1"))]
    OpenLdapPublicV1_1,
    #[strum(props(str = "oldap-1.2"))]
    OpenLdapPublicV1_2,
    #[strum(props(str = "oldap-1.3"))]
    OpenLdapPublicV1_3,
    #[strum(props(str = "oldap-1.4"))]
    OpenLdapPublicV1_4,
    #[strum(props(str = "oldap-2.0"))]
    OpenLdapPublicV2_0OrPossibly2_0AAnd2_0B,
    #[strum(props(str = "oldap-2.0.1"))]
    OpenLdapPublicV2_0_1,
    #[strum(props(str = "oldap-2.1"))]
    OpenLdapPublicV2_1,
    #[strum(props(str = "oldap-2.2"))]
    OpenLdapPublicV2_2,
    #[strum(props(str = "oldap-2.2.1"))]
    OpenLdapPublicV2_2_1,
    #[strum(props(str = "oldap-2.2.2"))]
    OpenLdapPublic2_2_2,
    #[strum(props(str = "oldap-2.3"))]
    OpenLdapPublicV2_3,
    #[strum(props(str = "oldap-2.4"))]
    OpenLdapPublicV2_4,
    #[strum(props(str = "oldap-2.5"))]
    OpenLdapPublicV2_5,
    #[strum(props(str = "oldap-2.6"))]
    OpenLdapPublicV2_6,
    #[strum(props(str = "oldap-2.7"))]
    OpenLdapPublicV2_7,
    #[strum(props(str = "oldap-2.8"))]
    OpenLdapPublicV2_8,
    #[strum(props(str = "oml"))]
    OpenMarket,
    #[strum(props(str = "openssl"))]
    OpenSsl,
    #[strum(props(str = "opl-1.0"))]
    OpenPublicV1_0,
    #[strum(props(str = "opubl-1.0"))]
    OpenPublicationV1_0,
    #[strum(props(str = "oset-pl-2.1"))]
    OsetPublicversion2_1,
    #[strum(props(str = "osl-1.0"))]
    OpenSoftware1_0,
    #[strum(props(str = "osl-1.1"))]
    OpenSoftware1_1,
    #[strum(props(str = "osl-2.0"))]
    OpenSoftware2_0,
    #[strum(props(str = "osl-2.1"))]
    OpenSoftware2_1,
    #[strum(props(str = "osl-3.0"))]
    OpenSoftware3_0,
    #[strum(props(str = "parity-6.0.0"))]
    TheParityPublic6_0_0,
    #[strum(props(str = "parity-7.0.0"))]
    TheParityPublic7_0_0,
    #[strum(props(str = "pddl-1.0"))]
    OpenDataCommonsPublicDomainDedication1_0,
    #[strum(props(str = "php-3.0"))]
    PhpV3_0,
    #[strum(props(str = "php-3.01"))]
    PhpV3_01,
    #[strum(props(str = "plexus"))]
    PlexusClassworlds,
    #[strum(props(str = "polyform-noncommercial-1.0.0"))]
    PolyFormNoncommercial1_0_0,
    #[strum(props(str = "polyform-small-business-1.0.0"))]
    PolyFormSmallBusiness1_0_0,
    #[strum(props(str = "postgresql"))]
    PostgreSql,
    #[strum(props(str = "psf-2.0"))]
    PythonSoftwareFoundation2_0,
    #[strum(props(str = "psfrag"))]
    Psfrag,
    #[strum(props(str = "psutils"))]
    Psutils,
    #[strum(props(str = "python-2.0"))]
    Python2_0,
    #[strum(props(str = "python-2.0.1"))]
    Python2_0_1,
    #[strum(props(str = "qhull"))]
    Qhull,
    #[strum(props(str = "qpl-1.0"))]
    QPublic1_0,
    #[strum(props(str = "rdisc"))]
    Rdisc,
    #[strum(props(str = "rhecos-1.1"))]
    RedHateCosPublicV1_1,
    #[strum(props(str = "rpl-1.1"))]
    ReciprocalPublic1_1,
    #[strum(props(str = "rpl-1.5"))]
    ReciprocalPublic1_5,
    #[strum(props(str = "rpsl-1.0"))]
    RealNetworksPublicSourceV1_0,
    #[strum(props(str = "rsa-md"))]
    RsaMessageDigest,
    #[strum(props(str = "rscpl"))]
    RicohSourceCodePublic,
    #[strum(props(str = "ruby"))]
    Ruby,
    #[strum(props(str = "sax-pd"))]
    SaxPublicDomainNotice,
    #[strum(props(str = "saxpath"))]
    Saxpath,
    #[strum(props(str = "scea"))]
    SceaSharedSource,
    #[strum(props(str = "schemereport"))]
    SchemeLanguageReport,
    #[strum(props(str = "sendmail"))]
    Sendmail,
    #[strum(props(str = "sendmail-8.23"))]
    Sendmail8_23,
    #[strum(props(str = "sgi-b-1.0"))]
    SgiFreeSoftwareBV1_0,
    #[strum(props(str = "sgi-b-1.1"))]
    SgiFreeSoftwareBV1_1,
    #[strum(props(str = "sgi-b-2.0"))]
    SgiFreeSoftwareBV2_0,
    #[strum(props(str = "shl-0.5"))]
    SolderpadHardwareV0_5,
    #[strum(props(str = "shl-0.51"))]
    SolderpadHardware,
    Version0_51,
    #[strum(props(str = "simpl-2.0"))]
    SimplePublic2_0,
    #[strum(props(str = "sissl"))]
    SunIndustryStandardsSourceV1_1,
    #[strum(props(str = "sissl-1.2"))]
    SunIndustryStandardsSourceV1_2,
    #[strum(props(str = "sleepycat"))]
    Sleepycat,
    #[strum(props(str = "smlnj"))]
    StandardMlOfNewJersey,
    #[strum(props(str = "smppl"))]
    SecureMessagingProtocolPublic,
    #[strum(props(str = "snia"))]
    SniaPublic1_1,
    #[strum(props(str = "spencer-86"))]
    Spencer86,
    #[strum(props(str = "spencer-94"))]
    Spencer94,
    #[strum(props(str = "spencer-99"))]
    Spencer99,
    #[strum(props(str = "spl-1.0"))]
    SunPublicV1_0,
    #[strum(props(str = "ssh-openssh"))]
    SshOpenSshlicense,
    #[strum(props(str = "ssh-short"))]
    SshShortNotice,
    #[strum(props(str = "sspl-1.0"))]
    ServerSidePublicV1,
    #[strum(props(str = "sugarcrm-1.1.3"))]
    SugarCrmPublicV1_1_3,
    #[strum(props(str = "swl"))]
    SchemeWidgetLibrarySwlSoftwareAgreement,
    #[strum(props(str = "tapr-ohl-1.0"))]
    TaprOpenHardwareV1_0,
    #[strum(props(str = "tcl"))]
    TclTk,
    #[strum(props(str = "tcp-wrappers"))]
    TcpWrappers,
    #[strum(props(str = "tmate"))]
    TMateOpenSource,
    #[strum(props(str = "torque-1.1"))]
    TorqueV2_5SoftwareV1_1,
    #[strum(props(str = "tosl"))]
    TrussterOpenSource,
    #[strum(props(str = "tu-berlin-1.0"))]
    TechnischeUniversitaetBerlin1_0,
    #[strum(props(str = "tu-berlin-2.0"))]
    TechnischeUniversitaetBerlin2_0,
    #[strum(props(str = "ucl-1.0"))]
    UpstreamCompatibilityV1_0,
    #[strum(props(str = "unicode-dfs-2015"))]
    UnicodeAgreementDataFilesAndSoftware2015,
    #[strum(props(str = "unicode-dfs-2016"))]
    UnicodeAgreementDataFilesAndSoftware2016,
    #[strum(props(str = "unicode-tou"))]
    UnicodeTermsofUse,
    #[strum(props(str = "unlicense"))]
    TheUnlicense,
    #[strum(props(str = "upl-1.0"))]
    UniversalPermissiveV1_0,
    #[strum(props(str = "vim"))]
    Vim,
    #[strum(props(str = "vostrom"))]
    VostromPublicforOpenSource,
    #[strum(props(str = "vsl-1.0"))]
    VovidaSoftwareV1_0,
    #[strum(props(str = "w3c"))]
    W3cSoftwareNoticeAnd20021231,
    #[strum(props(str = "w3c-19980720"))]
    W3cSoftwareNoticeAnd19980720,
    #[strum(props(str = "w3c-20150513"))]
    W3cSoftwareNoticeAndDocument20150513,
    #[strum(props(str = "watcom-1.0"))]
    SybaseOpenWatcomPublic1_0,
    #[strum(props(str = "wsuipa"))]
    Wsuipa,
    #[strum(props(str = "wtfpl"))]
    DoWhatTheFckYouWantToPublic,
    #[strum(props(str = "x11"))]
    X11License,
    #[strum(props(str = "x11-distribute-modifications-variant"))]
    X11LicenseDistributionModificationVariant,
    #[strum(props(str = "xerox"))]
    Xerox,
    #[strum(props(str = "xfree86-1.1"))]
    XFree86License1_1,
    #[strum(props(str = "xinetd"))]
    Xinetd,
    #[strum(props(str = "xnet"))]
    XNet,
    #[strum(props(str = "xpp"))]
    Xpp,
    #[strum(props(str = "xskat"))]
    XSkat,
    #[strum(props(str = "ypl-1.0"))]
    YahooPublicV1_0,
    #[strum(props(str = "ypl-1.1"))]
    YahooPublicV1_1,
    #[strum(props(str = "zed"))]
    Zed,
    #[strum(props(str = "zend-2.0"))]
    ZendV2_0,
    #[strum(props(str = "zimbra-1.3"))]
    ZimbraPublicV1_3,
    #[strum(props(str = "zimbra-1.4"))]
    ZimbraPublicV1_4,
    #[strum(props(str = "zlib"))]
    Zlib,
    #[strum(props(str = "zlib-acknowledgement"))]
    ZlibLibpngwithAcknowledgement,
    #[strum(props(str = "zpl-1.1"))]
    ZopePublic1_1,
    #[strum(props(str = "zpl-2.0"))]
    ZopePublic2_0,
    #[strum(props(str = "zpl-2.1"))]
    ZopePublic2_1,

    Other(String),
}

impl fmt::Display for LicenseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other(s) => write!(f, "{s}"),
            _ => match self.get_str("str") {
                Some(s) => write!(f, "{}", s),
                None => write!(f, "{:?}", self),
            },
        }
    }
}

impl<'de> Deserialize<'de> for LicenseType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        for variant in Self::iter() {
            if format!("{variant}") == s {
                return Ok(variant);
            };
        }

        Ok(Self::Other(s))
    }
}
