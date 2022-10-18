use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

// TODO: improve variant names.
#[derive(Debug, PartialEq, Eq, EnumString, Display)]
pub enum LicenseType {
    #[strum(serialize = "0bsd")]
    BsdZeroClause,
    #[strum(serialize = "aal")]
    AttributionAssurance,
    #[strum(serialize = "abstyles")]
    Abstyles,
    #[strum(serialize = "adobe-2006")]
    AdobeSystemsIncorporatedSourceCodeAgreement,
    #[strum(serialize = "adobe-glyph")]
    AdobeGlyphList,
    #[strum(serialize = "adsl")]
    AmazonDigitalServices,
    #[strum(serialize = "afl-1.1")]
    AcademicFreev1_1,
    #[strum(serialize = "afl-1.2")]
    AcademicFreev1_2,
    #[strum(serialize = "afl-2.0")]
    AcademicFreev2_0,
    #[strum(serialize = "afl-2.1")]
    AcademicFreev2_1,
    #[strum(serialize = "afl-3.0")]
    AcademicFreev3_0,
    #[strum(serialize = "afmparse")]
    Afmparse,
    #[strum(serialize = "agpl-1.0-only")]
    AfferoGeneralPublicv1_0only,
    #[strum(serialize = "agpl-1.0-or-later")]
    AfferoGeneralPublicv1_0OrLater,
    #[strum(serialize = "agpl-3.0-only")]
    GnuAfferoGeneralPublicv3_0only,
    #[strum(serialize = "agpl-3.0-or-later")]
    GnuAfferoGeneralPublicv3_0OrLater,
    #[strum(serialize = "aladdin")]
    AladdinFreePublic,
    #[strum(serialize = "amdplpa")]
    AmdPlpaMapC,
    #[strum(serialize = "aml")]
    AppleMit,
    #[strum(serialize = "ampas")]
    AcademyofMotionPictureArtsAndSciencesBsd,
    #[strum(serialize = "antlr-pd")]
    AntlrSoftwareRightsNotice,
    #[strum(serialize = "antlr-pd-fallback")]
    AntlrSoftwareRightsNoticewithlicensefallback,
    #[strum(serialize = "apache-1.0")]
    Apache1_0,
    #[strum(serialize = "apache-1.1")]
    Apache1_1,
    #[strum(serialize = "apache-2.0")]
    Apache2_0,
    #[strum(serialize = "apafml")]
    AdobePostscriptAfm,
    #[strum(serialize = "apl-1.0")]
    AdaptivePublic1_0,
    #[strum(serialize = "app-s2p")]
    AppS2p,
    #[strum(serialize = "apsl-1.0")]
    ApplePublicSource1_0,
    #[strum(serialize = "apsl-1.1")]
    ApplePublicSource1_1,
    #[strum(serialize = "apsl-1.2")]
    ApplePublicSource1_2,
    #[strum(serialize = "apsl-2.0")]
    ApplePublicSource2_0,
    #[strum(serialize = "arphic-1999")]
    ArphicPublic,
    #[strum(serialize = "artistic-1.0")]
    Artistic1_0,
    #[strum(serialize = "artistic-1.0-cl8")]
    Artistic1_0wClause8,
    #[strum(serialize = "artistic-1.0-perl")]
    Artistic1_0Perl,
    #[strum(serialize = "artistic-2.0")]
    Artistic2_0,
    #[strum(serialize = "baekmuk")]
    Baekmuk,
    #[strum(serialize = "bahyph")]
    Bahyph,
    #[strum(serialize = "barr")]
    Barr,
    #[strum(serialize = "beerware")]
    Beerware,
    #[strum(serialize = "bitstream-vera")]
    BitstreamVeraFont,
    #[strum(serialize = "bittorrent-1.0")]
    BitTorrentOpenSourcev1_0,
    #[strum(serialize = "bittorrent-1.1")]
    BitTorrentOpenSourcev1_1,
    #[strum(serialize = "blessing")]
    SqliteBlessing,
    #[strum(serialize = "blueoak-1.0.0")]
    BlueOakModel1_0_0,
    #[strum(serialize = "borceux")]
    Borceuxlicense,
    #[strum(serialize = "bsd-1-clause")]
    Bsd1Clause,
    #[strum(serialize = "bsd-2-clause")]
    Bsd2ClauseSimplified,
    #[strum(serialize = "bsd-2-clause-patent")]
    Bsd2ClausePlusPatent,
    #[strum(serialize = "bsd-2-clause-views")]
    Bsd2Clausewithviewssentence,
    #[strum(serialize = "bsd-3-clause")]
    Bsd3ClauseNewOrRevised,
    #[strum(serialize = "bsd-3-clause-attribution")]
    BsdWithAttribution,
    #[strum(serialize = "bsd-3-clause-clear")]
    Bsd3ClauseClear,
    #[strum(serialize = "bsd-3-clause-lbnl")]
    LawrenceBerkeleyNationalLabsBsdVariantlicense,
    #[strum(serialize = "bsd-3-clause-modification")]
    Bsd3ClauseModification,
    #[strum(serialize = "bsd-3-clause-no-military-license")]
    Bsd3ClauseNoMilitary,
    #[strum(serialize = "bsd-3-clause-no-nuclear-license")]
    Bsd3ClauseNoNuclear,
    #[strum(serialize = "bsd-3-clause-no-nuclear-license-2014")]
    Bsd3ClauseNoNuclear2014,
    #[strum(serialize = "bsd-3-clause-no-nuclear-warranty")]
    Bsd3ClauseNoNuclearWarranty,
    #[strum(serialize = "bsd-3-clause-open-mpi")]
    Bsd3ClauseOpenMpiVariant,
    #[strum(serialize = "bsd-4-clause")]
    Bsd4ClauseOriginalOrOld,
    #[strum(serialize = "bsd-4-clause-shortened")]
    Bsd4ClauseShortened,
    #[strum(serialize = "bsd-4-clause-uc")]
    Bsd4ClauseUniversityofCaliforniaSpecific,
    #[strum(serialize = "bsd-protection")]
    BsdProtection,
    #[strum(serialize = "bsd-source-code")]
    BsdSourceCodeAttribution,
    #[strum(serialize = "bsl-1.0")]
    BoostSoftware1_0,
    #[strum(serialize = "busl-1.1")]
    BusinessSource1_1,
    #[strum(serialize = "bzip2-1.0.6")]
    Bzip2AndLibbzip2Licensev1_0_6,
    #[strum(serialize = "c-uda-1.0")]
    ComputationalUseofDataAgreementv1_0,
    #[strum(serialize = "cal-1.0")]
    CryptographicAutonomy1_0,
    #[strum(serialize = "cal-1.0-combined-work-exception")]
    CryptographicAutonomy1_0CombinedWorkException,
    #[strum(serialize = "caldera")]
    Caldera,
    #[strum(serialize = "catosl-1.1")]
    ComputerAssociatesTrustedOpenSource1_1,
    #[strum(serialize = "cc-by-1.0")]
    CreativeCommonsAttribution1_0Generic,
    #[strum(serialize = "cc-by-2.0")]
    CreativeCommonsAttribution2_0Generic,
    #[strum(serialize = "cc-by-2.5")]
    CreativeCommonsAttribution2_5Generic,
    #[strum(serialize = "cc-by-2.5-au")]
    CreativeCommonsAttribution2_5Australia,
    #[strum(serialize = "cc-by-3.0")]
    CreativeCommonsAttribution3_0Unported,
    #[strum(serialize = "cc-by-3.0-at")]
    CreativeCommonsAttribution3_0Austria,
    #[strum(serialize = "cc-by-3.0-de")]
    CreativeCommonsAttribution3_0Germany,
    #[strum(serialize = "cc-by-3.0-igo")]
    CreativeCommonsAttribution3_0Igo,
    #[strum(serialize = "cc-by-3.0-nl")]
    CreativeCommonsAttribution3_0Netherlands,
    #[strum(serialize = "cc-by-3.0-us")]
    CreativeCommonsAttribution3_0UnitedStates,
    #[strum(serialize = "cc-by-4.0")]
    CreativeCommonsAttribution4_0International,
    #[strum(serialize = "cc-by-nc-1.0")]
    CreativeCommonsAttributionNonCommercial1_0Generic,
    #[strum(serialize = "cc-by-nc-2.0")]
    CreativeCommonsAttributionNonCommercial2_0Generic,
    #[strum(serialize = "cc-by-nc-2.5")]
    CreativeCommonsAttributionNonCommercial2_5Generic,
    #[strum(serialize = "cc-by-nc-3.0")]
    CreativeCommonsAttributionNonCommercial3_0Unported,
    #[strum(serialize = "cc-by-nc-3.0-de")]
    CreativeCommonsAttributionNonCommercial3_0Germany,
    #[strum(serialize = "cc-by-nc-4.0")]
    CreativeCommonsAttributionNonCommercial4_0International,
    #[strum(serialize = "cc-by-nc-nd-1.0")]
    CreativeCommonsAttributionNonCommercialNoDerivatives1_0Generic,
    #[strum(serialize = "cc-by-nc-nd-2.0")]
    CreativeCommonsAttributionNonCommercialNoDerivatives2_0Generic,
    #[strum(serialize = "cc-by-nc-nd-2.5")]
    CreativeCommonsAttributionNonCommercialNoDerivatives2_5Generic,
    #[strum(serialize = "cc-by-nc-nd-3.0")]
    CreativeCommonsAttributionNonCommercialNoDerivatives3_0Unported,
    #[strum(serialize = "cc-by-nc-nd-3.0-de")]
    CreativeCommonsAttributionNonCommercialNoDerivatives3_0Germany,
    #[strum(serialize = "cc-by-nc-nd-3.0-igo")]
    CreativeCommonsAttributionNonCommercialNoDerivatives3_0Igo,
    #[strum(serialize = "cc-by-nc-nd-4.0")]
    CreativeCommonsAttributionNonCommercialNoDerivatives4_0International,
    #[strum(serialize = "cc-by-nc-sa-1.0")]
    CreativeCommonsAttributionNonCommercialShareAlike1_0Generic,
    #[strum(serialize = "cc-by-nc-sa-2.0")]
    CreativeCommonsAttributionNonCommercialShareAlike2_0Generic,
    #[strum(serialize = "cc-by-nc-sa-2.0-fr")]
    CreativeCommonsAttributionNonCommercialShareAlike2_0France,
    #[strum(serialize = "cc-by-nc-sa-2.0-uk")]
    CreativeCommonsAttributionNonCommercialShareAlike2_0EnglandandWales,
    #[strum(serialize = "cc-by-nc-sa-2.5")]
    CreativeCommonsAttributionNonCommercialShareAlike2_5Generic,
    #[strum(serialize = "cc-by-nc-sa-3.0")]
    CreativeCommonsAttributionNonCommercialShareAlike3_0Unported,
    #[strum(serialize = "cc-by-nc-sa-3.0-de")]
    CreativeCommonsAttributionNonCommercialShareAlike3_0Germany,
    #[strum(serialize = "cc-by-nc-sa-3.0-igo")]
    CreativeCommonsAttributionNonCommercialShareAlike3_0Igo,
    #[strum(serialize = "cc-by-nc-sa-4.0")]
    CreativeCommonsAttributionNonCommercialShareAlike4_0International,
    #[strum(serialize = "cc-by-nd-1.0")]
    CreativeCommonsAttributionNoDerivatives1_0Generic,
    #[strum(serialize = "cc-by-nd-2.0")]
    CreativeCommonsAttributionNoDerivatives2_0Generic,
    #[strum(serialize = "cc-by-nd-2.5")]
    CreativeCommonsAttributionNoDerivatives2_5Generic,
    #[strum(serialize = "cc-by-nd-3.0")]
    CreativeCommonsAttributionNoDerivatives3_0Unported,
    #[strum(serialize = "cc-by-nd-3.0-de")]
    CreativeCommonsAttributionNoDerivatives3_0Germany,
    #[strum(serialize = "cc-by-nd-4.0")]
    CreativeCommonsAttributionNoDerivatives4_0International,
    #[strum(serialize = "cc-by-sa-1.0")]
    CreativeCommonsAttributionShareAlike1_0Generic,
    #[strum(serialize = "cc-by-sa-2.0")]
    CreativeCommonsAttributionShareAlike2_0Generic,
    #[strum(serialize = "cc-by-sa-2.0-uk")]
    CreativeCommonsAttributionShareAlike2_0EnglAndandWales,
    #[strum(serialize = "cc-by-sa-2.1-jp")]
    CreativeCommonsAttributionShareAlike2_1Japan,
    #[strum(serialize = "cc-by-sa-2.5")]
    CreativeCommonsAttributionShareAlike2_5Generic,
    #[strum(serialize = "cc-by-sa-3.0")]
    CreativeCommonsAttributionShareAlike3_0Unported,
    #[strum(serialize = "cc-by-sa-3.0-at")]
    CreativeCommonsAttributionShareAlike3_0Austria,
    #[strum(serialize = "cc-by-sa-3.0-de")]
    CreativeCommonsAttributionShareAlike3_0Germany,
    #[strum(serialize = "cc-by-sa-4.0")]
    CreativeCommonsAttributionShareAlike4_0International,
    #[strum(serialize = "cc-pddc")]
    CreativeCommonsPublicDomainDedicationAndCertification,
    #[strum(serialize = "cc0-1.0")]
    CreativeCommonsZerov1_0Universal,
    #[strum(serialize = "cddl-1.0")]
    CommonDevelopmentAndDistribution1_0,
    #[strum(serialize = "cddl-1.1")]
    CommonDevelopmentAndDistribution1_1,
    #[strum(serialize = "cdl-1.0")]
    CommonDocumentation1_0,
    #[strum(serialize = "cdla-permissive-1.0")]
    CommunityDataAgreementPermissive1_0,
    #[strum(serialize = "cdla-permissive-2.0")]
    CommunityDataAgreementPermissive2_0,
    #[strum(serialize = "cdla-sharing-1.0")]
    CommunityDataAgreementSharing1_0,
    #[strum(serialize = "cecill-1.0")]
    CeCillFreeSoftwareAgreementv1_0,
    #[strum(serialize = "cecill-1.1")]
    CeCillFreeSoftwareAgreementv1_1,
    #[strum(serialize = "cecill-2.0")]
    CeCillFreeSoftwareAgreementv2_0,
    #[strum(serialize = "cecill-2.1")]
    CeCillFreeSoftwareAgreementv2_1,
    #[strum(serialize = "cecill-b")]
    CeCillBFreeSoftwareAgreement,
    #[strum(serialize = "cecill-c")]
    CeCillCFreeSoftwareAgreement,
    #[strum(serialize = "cern-ohl-1.1")]
    CernOpenHardwareLicencev1_1,
    #[strum(serialize = "cern-ohl-1.2")]
    CernOpenHardwareLicencev1_2,
    #[strum(serialize = "cern-ohl-p-2.0")]
    CernOpenHardwareLicenceVersion2Permissive,
    #[strum(serialize = "cern-ohl-s-2.0")]
    CernOpenHardwareLicenceVersion2StronglyReciprocal,
    #[strum(serialize = "cern-ohl-w-2.0")]
    CernOpenHardwareLicenceVersion2WeaklyReciprocal,
    #[strum(serialize = "clartistic")]
    ClarifiedArtistic,
    #[strum(serialize = "cnri-jython")]
    CnriJython,
    #[strum(serialize = "cnri-python")]
    CnriPython,
    #[strum(serialize = "cnri-python-gpl-compatible")]
    CnriPythonOpenSourceGplCompatibleAgreement,
    #[strum(serialize = "coil-1.0")]
    CopyfreeOpenInnovation,
    #[strum(serialize = "community-spec-1.0")]
    CommunitySpecification1_0,
    #[strum(serialize = "condor-1.1")]
    CondorPublicv1_1,
    #[strum(serialize = "copyleft-next-0.3.0")]
    Copyleftnext0_3_0,
    #[strum(serialize = "copyleft-next-0.3.1")]
    Copyleftnext0_3_1,
    #[strum(serialize = "cpal-1.0")]
    CommonPublicAttribution1_0,
    #[strum(serialize = "cpl-1.0")]
    CommonPublic1_0,
    #[strum(serialize = "cpol-1.02")]
    CodeProjectOpen1_02,
    #[strum(serialize = "crossword")]
    Crossword,
    #[strum(serialize = "crystalstacker")]
    CrystalStacker,
    #[strum(serialize = "cua-opl-1.0")]
    CuaOfficePublicv1_0,
    #[strum(serialize = "cube")]
    Cube,
    #[strum(serialize = "curl")]
    Curl,
    #[strum(serialize = "d-fsl-1.0")]
    DeutscheFreieSoftwareLizenz,
    #[strum(serialize = "diffmark")]
    Diffmark,
    #[strum(serialize = "dl-de-by-2.0")]
    DatalicenceGermanyAttributionVersion2_0,
    #[strum(serialize = "doc")]
    Doc,
    #[strum(serialize = "dotseqn")]
    Dotseqn,
    #[strum(serialize = "drl-1.0")]
    DetectionRule1_0,
    #[strum(serialize = "dsdp")]
    Dsdp,
    #[strum(serialize = "dvipdfm")]
    Dvipdfm,
    #[strum(serialize = "ecl-1.0")]
    EducationalCommunityv1_0,
    #[strum(serialize = "ecl-2.0")]
    EducationalCommunityv2_0,
    #[strum(serialize = "efl-1.0")]
    EiffelForumv1_0,
    #[strum(serialize = "efl-2.0")]
    EiffelForumv2_0,
    #[strum(serialize = "egenix")]
    EGenixComPublic1_1_0,
    #[strum(serialize = "elastic-2.0")]
    Elastic2_0,
    #[strum(serialize = "entessa")]
    EntessaPublicv1_0,
    #[strum(serialize = "epics")]
    EpicsOpen,
    #[strum(serialize = "epl-1.0")]
    EclipsePublic1_0,
    #[strum(serialize = "epl-2.0")]
    EclipsePublic2_0,
    #[strum(serialize = "erlpl-1.1")]
    ErlangPublicv1_1,
    #[strum(serialize = "etalab-2.0")]
    EtalabOpen2_0,
    #[strum(serialize = "eudatagrid")]
    EuDataGridSoftware,
    #[strum(serialize = "eupl-1.0")]
    EuropeanUnionPublic1_0,
    #[strum(serialize = "eupl-1.1")]
    EuropeanUnionPublic1_1,
    #[strum(serialize = "eupl-1.2")]
    EuropeanUnionPublic1_2,
    #[strum(serialize = "eurosym")]
    Eurosym,
    #[strum(serialize = "fair")]
    Fair,
    #[strum(serialize = "fdk-aac")]
    FraunhoferFdkAacCodecLibrary,
    #[strum(serialize = "frameworx-1.0")]
    FrameworxOpen1_0,
    #[strum(serialize = "freebsd-doc")]
    FreeBsdDocumentation,
    #[strum(serialize = "freeimage")]
    FreeImagePublicv1_0,
    #[strum(serialize = "fsfap")]
    FsfAllPermissive,
    #[strum(serialize = "fsful")]
    FsfUnlimited,
    #[strum(serialize = "fsfullr")]
    FsfUnlimitedWithLicenseRetention,
    #[strum(serialize = "ftl")]
    FreetypeProject,
    #[strum(serialize = "gd")]
    Gd,
    #[strum(serialize = "gfdl-1.1-invariants-only")]
    GnuFreeDocumentationv1_1OnlyInvariants,
    #[strum(serialize = "gfdl-1.1-invariants-or-later")]
    GnuFreeDocumentationv1_1OrLaterInvariants,
    #[strum(serialize = "gfdl-1.1-no-invariants-only")]
    GnuFreeDocumentationv1_1OnlyNoInvariants,
    #[strum(serialize = "gfdl-1.1-no-invariants-or-later")]
    GnuFreeDocumentationv1_1OrLaterNoInvariants,
    #[strum(serialize = "gfdl-1.1-only")]
    GnuFreeDocumentationv1_1Only,
    #[strum(serialize = "gfdl-1.1-or-later")]
    GnuFreeDocumentationv1_1OrLater,
    #[strum(serialize = "gfdl-1.2-invariants-only")]
    GnuFreeDocumentationv1_2OnlyInvariants,
    #[strum(serialize = "gfdl-1.2-invariants-or-later")]
    GnuFreeDocumentationv1_2OrLaterInvariants,
    #[strum(serialize = "gfdl-1.2-no-invariants-only")]
    GnuFreeDocumentationv1_2OnlyNoInvariants,
    #[strum(serialize = "gfdl-1.2-no-invariants-or-later")]
    GnuFreeDocumentationv1_2OrLaterNoInvariants,
    #[strum(serialize = "gfdl-1.2-only")]
    GnuFreeDocumentationv1_2Only,
    #[strum(serialize = "gfdl-1.2-or-later")]
    GnuFreeDocumentationv1_2OrLater,
    #[strum(serialize = "gfdl-1.3-invariants-only")]
    GnuFreeDocumentationv1_3OnlyInvariants,
    #[strum(serialize = "gfdl-1.3-invariants-or-later")]
    GnuFreeDocumentationv1_3OrLaterInvariants,
    #[strum(serialize = "gfdl-1.3-no-invariants-only")]
    GnuFreeDocumentationv1_3OnlyNoInvariants,
    #[strum(serialize = "gfdl-1.3-no-invariants-or-later")]
    GnuFreeDocumentationv1_3OrLaterNoInvariants,
    #[strum(serialize = "gfdl-1.3-only")]
    GnuFreeDocumentationv1_3Only,
    #[strum(serialize = "gfdl-1.3-or-later")]
    GnuFreeDocumentationv1_3OrLater,
    #[strum(serialize = "giftware")]
    Giftware,
    #[strum(serialize = "gl2ps")]
    Gl2ps,
    #[strum(serialize = "glide")]
    DfxGlide,
    #[strum(serialize = "glulxe")]
    Glulxe,
    #[strum(serialize = "glwtpl")]
    GoodLuckWithThatPublic,
    #[strum(serialize = "gnuplot")]
    GnuPlot,
    #[strum(serialize = "gpl-1.0-only")]
    GnuGeneralPublicv1_0only,
    #[strum(serialize = "gpl-1.0-or-later")]
    GnuGeneralPublicv1_0OrLater,
    #[strum(serialize = "gpl-2.0-only")]
    GnuGeneralPublicv2_0only,
    #[strum(serialize = "gpl-2.0-or-later")]
    GnuGeneralPublicv2_0OrLater,
    #[strum(serialize = "gpl-3.0-only")]
    GnuGeneralPublicv3_0only,
    #[strum(serialize = "gpl-3.0-or-later")]
    GnuGeneralPublicv3_0OrLater,
    #[strum(serialize = "gsoap-1.3b")]
    GsoapPublicv1_3b,
    #[strum(serialize = "haskellreport")]
    HaskellLanguageReport,
    #[strum(serialize = "hippocratic-2.1")]
    Hippocratic2_1,
    #[strum(serialize = "hpnd")]
    HistoricalPermissionNoticeAndDisclaimer,
    #[strum(serialize = "hpnd-sell-variant")]
    HistoricalPermissionNoticeAndDisclaimersellvariant,
    #[strum(serialize = "htmltidy")]
    HtmlTidy,
    #[strum(serialize = "ibm-pibs")]
    IbmPowerPcInitializationAndBootSoftware,
    #[strum(serialize = "icu")]
    Icu,
    #[strum(serialize = "ijg")]
    IndependentJpegGroup,
    #[strum(serialize = "imagemagick")]
    ImageMagick,
    #[strum(serialize = "imatix")]
    IMatixStAndardFunctionLibraryAgreement,
    #[strum(serialize = "imlib2")]
    Imlib2License,
    #[strum(serialize = "info-zip")]
    InfoZip,
    #[strum(serialize = "intel")]
    IntelOpenSource,
    #[strum(serialize = "intel-acpi")]
    IntelAcpiSoftwareAgreement,
    #[strum(serialize = "interbase-1.0")]
    InterbasePublicv1_0,
    #[strum(serialize = "ipa")]
    IpaFont,
    #[strum(serialize = "ipl-1.0")]
    IbmPublicv1_0,
    #[strum(serialize = "isc")]
    Isc,
    #[strum(serialize = "jam")]
    Jam,
    #[strum(serialize = "jasper-2.0")]
    JasPer,
    #[strum(serialize = "jpnic")]
    JapanNetworkInformationCenter,
    #[strum(serialize = "json")]
    Json,
    #[strum(serialize = "lal-1.2")]
    LicenceArtLibre1_2,
    #[strum(serialize = "lal-1.3")]
    LicenceArtLibre1_3,
    #[strum(serialize = "latex2e")]
    Latex2e,
    #[strum(serialize = "leptonica")]
    Leptonica,
    #[strum(serialize = "lgpl-2.0-only")]
    GnuLibraryGeneralPublicV2only,
    #[strum(serialize = "lgpl-2.0-or-later")]
    GnuLibraryGeneralPublicV2OrLater,
    #[strum(serialize = "lgpl-2.1-only")]
    GnuLesserGeneralPublicV2_1only,
    #[strum(serialize = "lgpl-2.1-or-later")]
    GnuLesserGeneralPublicV2_1OrLater,
    #[strum(serialize = "lgpl-3.0-only")]
    GnuLesserGeneralPublicV3_0Only,
    #[strum(serialize = "lgpl-3.0-or-later")]
    GnuLesserGeneralPublicv3_0OrLater,
    #[strum(serialize = "lgpllr")]
    LesserGeneralPublicForLinguisticResources,
    #[strum(serialize = "libpng")]
    Libpng,
    #[strum(serialize = "libpng-2.0")]
    PngReferenceLibraryversion2,
    #[strum(serialize = "libselinux-1.0")]
    LibselinuxPublicDomaiNnotice,
    #[strum(serialize = "libtiff")]
    Libtiff,
    #[strum(serialize = "liliq-p-1.1")]
    LicenceLibreduQuebecPermissiveversion1_1,
    #[strum(serialize = "liliq-r-1.1")]
    LicenceLibreduQuebecReciprociteversion1_1,
    #[strum(serialize = "liliq-rplus-1.1")]
    LicenceLibreduQuebecReciprociteforteversion1_1,
    #[strum(serialize = "linux-man-pages-copyleft")]
    LinuxmanpagesCopyleft,
    #[strum(serialize = "linux-openib")]
    LinuxKernelVariantofOpenIbOrg,
    #[strum(serialize = "lpl-1.0")]
    LucentPublicVersion1_0,
    #[strum(serialize = "lpl-1.02")]
    LucentPublicv1_02,
    #[strum(serialize = "lppl-1.0")]
    LaTeXProjectPublicv1_0,
    #[strum(serialize = "lppl-1.1")]
    LaTeXProjectPublicv1_1,
    #[strum(serialize = "lppl-1.2")]
    LaTeXProjectPublicv1_2,
    #[strum(serialize = "lppl-1.3a")]
    LaTeXProjectPublicv1_3a,
    #[strum(serialize = "lppl-1.3c")]
    LaTeXProjectPublicv1_3c,
    #[strum(serialize = "lzma-sdk-9.11-to-9.20")]
    LzmaSdkVersions9_11To9_20,
    #[strum(serialize = "lzma-sdk-9.22")]
    LzmaSdkVersions9_22AndBeyond,
    #[strum(serialize = "makeindex")]
    MakeIndex,
    #[strum(serialize = "minpack")]
    Minpack,
    #[strum(serialize = "miros")]
    TheMirOsLicence,
    #[strum(serialize = "mit")]
    Mit,
    #[strum(serialize = "mit-0")]
    MitNoAttribution,
    #[strum(serialize = "mit-advertising")]
    EnlightenmentE16,
    #[strum(serialize = "mit-cmu")]
    Cmu,
    #[strum(serialize = "mit-enna")]
    Enna,
    #[strum(serialize = "mit-feh")]
    Feh,
    #[strum(serialize = "mit-modern-variant")]
    MitModernVariant,
    #[strum(serialize = "mit-open-group")]
    MitOpenGroupvariant,
    #[strum(serialize = "mitnfa")]
    MitNoFalseAttribs,
    #[strum(serialize = "motosoto")]
    Motosoto,
    #[strum(serialize = "mpi-permissive")]
    MpiPermissive,
    #[strum(serialize = "mpich2")]
    Mpich2License,
    #[strum(serialize = "mpl-1.0")]
    MozillaPublic1_0,
    #[strum(serialize = "mpl-1.1")]
    MozillaPublic1_1,
    #[strum(serialize = "mpl-2.0")]
    MozillaPublic2_0,
    #[strum(serialize = "mpl-2.0-no-copyleft-exception")]
    MozillaPublic2_0nocopyleftexception,
    #[strum(serialize = "mplus")]
    MplusFont,
    #[strum(serialize = "ms-lpl")]
    MicrosoftLimitedPublic,
    #[strum(serialize = "ms-pl")]
    MicrosoftPublic,
    #[strum(serialize = "ms-rl")]
    MicrosoftReciprocal,
    #[strum(serialize = "mtll")]
    MatrixTemplateLibrary,
    #[strum(serialize = "mulanpsl-1.0")]
    MulanPermissiveSoftwareVersion1,
    #[strum(serialize = "mulanpsl-2.0")]
    MulanPermissiveSoftwareVersion2,
    #[strum(serialize = "multics")]
    Multics,
    #[strum(serialize = "mup")]
    Mup,
    #[strum(serialize = "naist-2003")]
    NaraInstituteofScienceAndTechnology2003,
    #[strum(serialize = "nasa-1.3")]
    NasaOpenSourceAgreement1_3,
    #[strum(serialize = "naumen")]
    NaumenPublic,
    #[strum(serialize = "nbpl-1.0")]
    NetBooleanPublicv1,
    #[strum(serialize = "ncgl-uk-2.0")]
    NonCommercialGovernmentLicence,
    #[strum(serialize = "ncsa")]
    UniversityofIllinoisNcsaOpenSource,
    #[strum(serialize = "net-snmp")]
    NetSnmp,
    #[strum(serialize = "netcdf")]
    NetCdf,
    #[strum(serialize = "newsletr")]
    Newsletr,
    #[strum(serialize = "ngpl")]
    NethackGeneralPublic,
    #[strum(serialize = "nicta-1.0")]
    NictaPublicSoftware,
    Version1_0,
    #[strum(serialize = "nist-pd")]
    NistPublicDomainNotice,
    #[strum(serialize = "nist-pd-fallback")]
    NistPublicDomainNoticewithlicensefallback,
    #[strum(serialize = "nlod-1.0")]
    NorwegianLicenceforOpenGovernmentDataNlod1_0,
    #[strum(serialize = "nlod-2.0")]
    NorwegianLicenceforOpenGovernmentDataNlod2_0,
    #[strum(serialize = "nlpl")]
    NoLimitPublic,
    #[strum(serialize = "nokia")]
    NokiaOpenSource,
    #[strum(serialize = "nosl")]
    NetizenOpenSource,
    #[strum(serialize = "noweb")]
    Noweb,
    #[strum(serialize = "npl-1.0")]
    NetscapePublicv1_0,
    #[strum(serialize = "npl-1.1")]
    NetscapePublicv1_1,
    #[strum(serialize = "nposl-3.0")]
    NonProfitOpenSoftware3_0,
    #[strum(serialize = "nrl")]
    Nrl,
    #[strum(serialize = "ntp")]
    Ntp,
    #[strum(serialize = "ntp-0")]
    NtpNoAttribution,
    #[strum(serialize = "o-uda-1.0")]
    OpenUseofDataAgreementv1_0,
    #[strum(serialize = "occt-pl")]
    OpenCascadeTechnologyPublic,
    #[strum(serialize = "oclc-2.0")]
    OclcResearchPublic2_0,
    #[strum(serialize = "odbl-1.0")]
    OpenDataCommonsOpenDatabasev1_0,
    #[strum(serialize = "odc-by-1.0")]
    OpenDataCommonsAttributionv1_0,
    #[strum(serialize = "ofl-1.0")]
    SilOpenFont1_0,
    #[strum(serialize = "ofl-1.0-no-rfn")]
    SilOpenFont1_0withnoReservedFontName,
    #[strum(serialize = "ofl-1.0-rfn")]
    SilOpenFont1_0withReservedFontName,
    #[strum(serialize = "ofl-1.1")]
    SilOpenFont1_1,
    #[strum(serialize = "ofl-1.1-no-rfn")]
    SilOpenFont1_1withnoReservedFontName,
    #[strum(serialize = "ofl-1.1-rfn")]
    SilOpenFont1_1withReservedFontName,
    #[strum(serialize = "ogc-1.0")]
    OgcSoftwareVersion1_0,
    #[strum(serialize = "ogdl-taiwan-1.0")]
    TaiwanOpenGovernmentDataVersion1_0,
    #[strum(serialize = "ogl-canada-2.0")]
    OpenGovernmentLicenceCanada,
    #[strum(serialize = "ogl-uk-1.0")]
    OpenGovernmentLicencev1_0,
    #[strum(serialize = "ogl-uk-2.0")]
    OpenGovernmentLicencev2_0,
    #[strum(serialize = "ogl-uk-3.0")]
    OpenGovernmentLicencev3_0,
    #[strum(serialize = "ogtsl")]
    OpenGroupTestSuite,
    #[strum(serialize = "oldap-1.1")]
    OpenLdapPublicv1_1,
    #[strum(serialize = "oldap-1.2")]
    OpenLdapPublicv1_2,
    #[strum(serialize = "oldap-1.3")]
    OpenLdapPublicv1_3,
    #[strum(serialize = "oldap-1.4")]
    OpenLdapPublicv1_4,
    #[strum(serialize = "oldap-2.0")]
    OpenLdapPublicv2_0OrPossibly2_0AAnd2_0B,
    #[strum(serialize = "oldap-2.0.1")]
    OpenLdapPublicv2_0_1,
    #[strum(serialize = "oldap-2.1")]
    OpenLdapPublicv2_1,
    #[strum(serialize = "oldap-2.2")]
    OpenLdapPublicv2_2,
    #[strum(serialize = "oldap-2.2.1")]
    OpenLdapPublicv2_2_1,
    #[strum(serialize = "oldap-2.2.2")]
    OpenLdapPublic2_2_2,
    #[strum(serialize = "oldap-2.3")]
    OpenLdapPublicv2_3,
    #[strum(serialize = "oldap-2.4")]
    OpenLdapPublicv2_4,
    #[strum(serialize = "oldap-2.5")]
    OpenLdapPublicv2_5,
    #[strum(serialize = "oldap-2.6")]
    OpenLdapPublicv2_6,
    #[strum(serialize = "oldap-2.7")]
    OpenLdapPublicv2_7,
    #[strum(serialize = "oldap-2.8")]
    OpenLdapPublicv2_8,
    #[strum(serialize = "oml")]
    OpenMarket,
    #[strum(serialize = "openssl")]
    OpenSsl,
    #[strum(serialize = "opl-1.0")]
    OpenPublicv1_0,
    #[strum(serialize = "opubl-1.0")]
    OpenPublicationv1_0,
    #[strum(serialize = "oset-pl-2.1")]
    OsetPublicversion2_1,
    #[strum(serialize = "osl-1.0")]
    OpenSoftware1_0,
    #[strum(serialize = "osl-1.1")]
    OpenSoftware1_1,
    #[strum(serialize = "osl-2.0")]
    OpenSoftware2_0,
    #[strum(serialize = "osl-2.1")]
    OpenSoftware2_1,
    #[strum(serialize = "osl-3.0")]
    OpenSoftware3_0,
    #[strum(serialize = "parity-6.0.0")]
    TheParityPublic6_0_0,
    #[strum(serialize = "parity-7.0.0")]
    TheParityPublic7_0_0,
    #[strum(serialize = "pddl-1.0")]
    OpenDataCommonsPublicDomainDedication1_0,
    #[strum(serialize = "php-3.0")]
    Phpv3_0,
    #[strum(serialize = "php-3.01")]
    Phpv3_01,
    #[strum(serialize = "plexus")]
    PlexusClassworlds,
    #[strum(serialize = "polyform-noncommercial-1.0.0")]
    PolyFormNoncommercial1_0_0,
    #[strum(serialize = "polyform-small-business-1.0.0")]
    PolyFormSmallBusiness1_0_0,
    #[strum(serialize = "postgresql")]
    PostgreSql,
    #[strum(serialize = "psf-2.0")]
    PythonSoftwareFoundation2_0,
    #[strum(serialize = "psfrag")]
    Psfrag,
    #[strum(serialize = "psutils")]
    Psutils,
    #[strum(serialize = "python-2.0")]
    Python2_0,
    #[strum(serialize = "python-2.0.1")]
    Python2_0_1,
    #[strum(serialize = "qhull")]
    Qhull,
    #[strum(serialize = "qpl-1.0")]
    QPublic1_0,
    #[strum(serialize = "rdisc")]
    Rdisc,
    #[strum(serialize = "rhecos-1.1")]
    RedHateCosPublicv1_1,
    #[strum(serialize = "rpl-1.1")]
    ReciprocalPublic1_1,
    #[strum(serialize = "rpl-1.5")]
    ReciprocalPublic1_5,
    #[strum(serialize = "rpsl-1.0")]
    RealNetworksPublicSourcev1_0,
    #[strum(serialize = "rsa-md")]
    RsaMessageDigest,
    #[strum(serialize = "rscpl")]
    RicohSourceCodePublic,
    #[strum(serialize = "ruby")]
    Ruby,
    #[strum(serialize = "sax-pd")]
    SaxPublicDomainNotice,
    #[strum(serialize = "saxpath")]
    Saxpath,
    #[strum(serialize = "scea")]
    SceaSharedSource,
    #[strum(serialize = "schemereport")]
    SchemeLanguageReport,
    #[strum(serialize = "sendmail")]
    Sendmail,
    #[strum(serialize = "sendmail-8.23")]
    Sendmail8_23,
    #[strum(serialize = "sgi-b-1.0")]
    SgiFreeSoftwareBv1_0,
    #[strum(serialize = "sgi-b-1.1")]
    SgiFreeSoftwareBv1_1,
    #[strum(serialize = "sgi-b-2.0")]
    SgiFreeSoftwareBv2_0,
    #[strum(serialize = "shl-0.5")]
    SolderpadHardwarev0_5,
    #[strum(serialize = "shl-0.51")]
    SolderpadHardware,
    Version0_51,
    #[strum(serialize = "simpl-2.0")]
    SimplePublic2_0,
    #[strum(serialize = "sissl")]
    SunIndustryStandardsSourcev1_1,
    #[strum(serialize = "sissl-1.2")]
    SunIndustryStandardsSourcev1_2,
    #[strum(serialize = "sleepycat")]
    Sleepycat,
    #[strum(serialize = "smlnj")]
    StandardMlOfNewJersey,
    #[strum(serialize = "smppl")]
    SecureMessagingProtocolPublic,
    #[strum(serialize = "snia")]
    SniaPublic1_1,
    #[strum(serialize = "spencer-86")]
    Spencer86,
    #[strum(serialize = "spencer-94")]
    Spencer94,
    #[strum(serialize = "spencer-99")]
    Spencer99,
    #[strum(serialize = "spl-1.0")]
    SunPublicv1_0,
    #[strum(serialize = "ssh-openssh")]
    SshOpenSshlicense,
    #[strum(serialize = "ssh-short")]
    SshShortNotice,
    #[strum(serialize = "sspl-1.0")]
    ServerSidePublicv1,
    #[strum(serialize = "sugarcrm-1.1.3")]
    SugarCrmPublicv1_1_3,
    #[strum(serialize = "swl")]
    SchemeWidgetLibrarySwlSoftwareAgreement,
    #[strum(serialize = "tapr-ohl-1.0")]
    TaprOpenHardwarev1_0,
    #[strum(serialize = "tcl")]
    TclTk,
    #[strum(serialize = "tcp-wrappers")]
    TcpWrappers,
    #[strum(serialize = "tmate")]
    TMateOpenSource,
    #[strum(serialize = "torque-1.1")]
    TorqueV2_5Softwarev1_1,
    #[strum(serialize = "tosl")]
    TrussterOpenSource,
    #[strum(serialize = "tu-berlin-1.0")]
    TechnischeUniversitaetBerlin1_0,
    #[strum(serialize = "tu-berlin-2.0")]
    TechnischeUniversitaetBerlin2_0,
    #[strum(serialize = "ucl-1.0")]
    UpstreamCompatibilityv1_0,
    #[strum(serialize = "unicode-dfs-2015")]
    UnicodeAgreementDataFilesAndSoftware2015,
    #[strum(serialize = "unicode-dfs-2016")]
    UnicodeAgreementDataFilesAndSoftware2016,
    #[strum(serialize = "unicode-tou")]
    UnicodeTermsofUse,
    #[strum(serialize = "unlicense")]
    TheUnlicense,
    #[strum(serialize = "upl-1.0")]
    UniversalPermissivev1_0,
    #[strum(serialize = "vim")]
    Vim,
    #[strum(serialize = "vostrom")]
    VostromPublicforOpenSource,
    #[strum(serialize = "vsl-1.0")]
    VovidaSoftwarev1_0,
    #[strum(serialize = "w3c")]
    W3cSoftwareNoticeAnd20021231,
    #[strum(serialize = "w3c-19980720")]
    W3cSoftwareNoticeAnd19980720,
    #[strum(serialize = "w3c-20150513")]
    W3cSoftwareNoticeAndDocument20150513,
    #[strum(serialize = "watcom-1.0")]
    SybaseOpenWatcomPublic1_0,
    #[strum(serialize = "wsuipa")]
    Wsuipa,
    #[strum(serialize = "wtfpl")]
    DoWhatTheFckYouWantToPublic,
    #[strum(serialize = "x11")]
    X11License,
    #[strum(serialize = "x11-distribute-modifications-variant")]
    X11LicenseDistributionModificationVariant,
    #[strum(serialize = "xerox")]
    Xerox,
    #[strum(serialize = "xfree86-1.1")]
    XFree86License1_1,
    #[strum(serialize = "xinetd")]
    Xinetd,
    #[strum(serialize = "xnet")]
    XNet,
    #[strum(serialize = "xpp")]
    Xpp,
    #[strum(serialize = "xskat")]
    XSkat,
    #[strum(serialize = "ypl-1.0")]
    YahooPublicv1_0,
    #[strum(serialize = "ypl-1.1")]
    YahooPublicv1_1,
    #[strum(serialize = "zed")]
    Zed,
    #[strum(serialize = "zend-2.0")]
    Zendv2_0,
    #[strum(serialize = "zimbra-1.3")]
    ZimbraPublicv1_3,
    #[strum(serialize = "zimbra-1.4")]
    ZimbraPublicv1_4,
    #[strum(serialize = "zlib")]
    Zlib,
    #[strum(serialize = "zlib-acknowledgement")]
    ZlibLibpngwithAcknowledgement,
    #[strum(serialize = "zpl-1.1")]
    ZopePublic1_1,
    #[strum(serialize = "zpl-2.0")]
    ZopePublic2_0,
    #[strum(serialize = "zpl-2.1")]
    ZopePublic2_1,

    #[strum(disabled)]
    Other(String),
}

impl<'de> Deserialize<'de> for LicenseType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match LicenseType::from_str(s.as_str()) {
            Ok(x) => Ok(x),
            Err(_) => Ok(LicenseType::Other(s)),
        }
    }
}
