use serde_enum_str::Deserialize_enum_str;

// TODO: improve variant names.
#[derive(Debug, Deserialize_enum_str, PartialEq)]
pub enum LicenseType {
    #[serde(rename = "0bsd")]
    BSDZeroClauseLicense,
    #[serde(rename = "aal")]
    AttributionAssuranceLicense,
    #[serde(rename = "abstyles")]
    AbstylesLicense,
    #[serde(rename = "adobe-2006")]
    AdobeSystemsIncorporatedSourceCodeLicenseAgreement,
    #[serde(rename = "adobe-glyph")]
    AdobeGlyphListLicense,
    #[serde(rename = "adsl")]
    AmazonDigitalServicesLicense,
    #[serde(rename = "afl-1.1")]
    AcademicFreeLicensev1_1,
    #[serde(rename = "afl-1.2")]
    AcademicFreeLicensev1_2,
    #[serde(rename = "afl-2.0")]
    AcademicFreeLicensev2_0,
    #[serde(rename = "afl-2.1")]
    AcademicFreeLicensev2_1,
    #[serde(rename = "afl-3.0")]
    AcademicFreeLicensev3_0,
    #[serde(rename = "afmparse")]
    AfmparseLicense,
    #[serde(rename = "agpl-1.0-only")]
    AfferoGeneralPublicLicensev1_0only,
    #[serde(rename = "agpl-1.0-or-later")]
    AfferoGeneralPublicLicensev1_0orlater,
    #[serde(rename = "agpl-3.0-only")]
    GNUAfferoGeneralPublicLicensev3_0only,
    #[serde(rename = "agpl-3.0-or-later")]
    GNUAfferoGeneralPublicLicensev3_0orlater,
    #[serde(rename = "aladdin")]
    AladdinFreePublicLicense,
    #[serde(rename = "amdplpa")]
    AMDPlpaMapCLicense,
    #[serde(rename = "aml")]
    AppleMITLicense,
    #[serde(rename = "ampas")]
    AcademyofMotionPictureArtsandSciencesBSD,
    #[serde(rename = "antlr-pd")]
    ANTLRSoftwareRightsNotice,
    #[serde(rename = "antlr-pd-fallback")]
    ANTLRSoftwareRightsNoticewithlicensefallback,
    #[serde(rename = "apache-1.0")]
    ApacheLicense1_0,
    #[serde(rename = "apache-1.1")]
    ApacheLicense1_1,
    #[serde(rename = "apache-2.0")]
    ApacheLicense2_0,
    #[serde(rename = "apafml")]
    AdobePostscriptAFMLicense,
    #[serde(rename = "apl-1.0")]
    AdaptivePublicLicense1_0,
    #[serde(rename = "app-s2p")]
    AppS2pLicense,
    #[serde(rename = "apsl-1.0")]
    ApplePublicSourceLicense1_0,
    #[serde(rename = "apsl-1.1")]
    ApplePublicSourceLicense1_1,
    #[serde(rename = "apsl-1.2")]
    ApplePublicSourceLicense1_2,
    #[serde(rename = "apsl-2.0")]
    ApplePublicSourceLicense2_0,
    #[serde(rename = "arphic-1999")]
    ArphicPublicLicense,
    #[serde(rename = "artistic-1.0")]
    ArtisticLicense1_0,
    #[serde(rename = "artistic-1.0-cl8")]
    ArtisticLicense1_0wClause8,
    #[serde(rename = "artistic-1.0-perl")]
    ArtisticLicense1_0Perl,
    #[serde(rename = "artistic-2.0")]
    ArtisticLicense2_0,
    #[serde(rename = "baekmuk")]
    BaekmukLicense,
    #[serde(rename = "bahyph")]
    BahyphLicense,
    #[serde(rename = "barr")]
    BarrLicense,
    #[serde(rename = "beerware")]
    BeerwareLicense,
    #[serde(rename = "bitstream-vera")]
    BitstreamVeraFontLicense,
    #[serde(rename = "bittorrent-1.0")]
    BitTorrentOpenSourceLicensev1_0,
    #[serde(rename = "bittorrent-1.1")]
    BitTorrentOpenSourceLicensev1_1,
    #[serde(rename = "blessing")]
    SQLiteBlessing,
    #[serde(rename = "blueoak-1.0.0")]
    BlueOakModelLicense1_0_0,
    #[serde(rename = "borceux")]
    Borceuxlicense,
    #[serde(rename = "bsd-1-clause")]
    BSD1ClauseLicense,
    #[serde(rename = "bsd-2-clause")]
    BSD2ClauseSimplifiedLicense,
    #[serde(rename = "bsd-2-clause-patent")]
    BSD2ClausePlusPatentLicense,
    #[serde(rename = "bsd-2-clause-views")]
    BSD2Clausewithviewssentence,
    #[serde(rename = "bsd-3-clause")]
    BSD3ClauseNewOrRevisedLicense,
    #[serde(rename = "bsd-3-clause-attribution")]
    BSDwithattribution,
    #[serde(rename = "bsd-3-clause-clear")]
    BSD3ClauseClearLicense,
    #[serde(rename = "bsd-3-clause-lbnl")]
    LawrenceBerkeleyNationalLabsBSDvariantlicense,
    #[serde(rename = "bsd-3-clause-modification")]
    BSD3ClauseModification,
    #[serde(rename = "bsd-3-clause-no-military-license")]
    BSD3ClauseNoMilitaryLicense,
    #[serde(rename = "bsd-3-clause-no-nuclear-license")]
    BSD3ClauseNoNuclearLicense,
    #[serde(rename = "bsd-3-clause-no-nuclear-license-2014")]
    BSD3ClauseNoNuclearLicense2014,
    #[serde(rename = "bsd-3-clause-no-nuclear-warranty")]
    BSD3ClauseNoNuclearWarranty,
    #[serde(rename = "bsd-3-clause-open-mpi")]
    BSD3ClauseOpenMPIvariant,
    #[serde(rename = "bsd-4-clause")]
    BSD4ClauseOriginalOrOldLicense,
    #[serde(rename = "bsd-4-clause-shortened")]
    BSD4ClauseShortened,
    #[serde(rename = "bsd-4-clause-uc")]
    BSD4ClauseUniversityofCaliforniaSpecific,
    #[serde(rename = "bsd-protection")]
    BSDProtectionLicense,
    #[serde(rename = "bsd-source-code")]
    BSDSourceCodeAttribution,
    #[serde(rename = "bsl-1.0")]
    BoostSoftwareLicense1_0,
    #[serde(rename = "busl-1.1")]
    BusinessSourceLicense1_1,
    #[serde(rename = "bzip2-1.0.6")]
    Bzip2AndLibbzip2Licensev1_0_6,
    #[serde(rename = "c-uda-1.0")]
    ComputationalUseofDataAgreementv1_0,
    #[serde(rename = "cal-1.0")]
    CryptographicAutonomyLicense1_0,
    #[serde(rename = "cal-1.0-combined-work-exception")]
    CryptographicAutonomyLicense1_0CombinedWorkException,
    #[serde(rename = "caldera")]
    CalderaLicense,
    #[serde(rename = "catosl-1.1")]
    ComputerAssociatesTrustedOpenSourceLicense1_1,
    #[serde(rename = "cc-by-1.0")]
    CreativeCommonsAttribution1_0Generic,
    #[serde(rename = "cc-by-2.0")]
    CreativeCommonsAttribution2_0Generic,
    #[serde(rename = "cc-by-2.5")]
    CreativeCommonsAttribution2_5Generic,
    #[serde(rename = "cc-by-2.5-au")]
    CreativeCommonsAttribution2_5Australia,
    #[serde(rename = "cc-by-3.0")]
    CreativeCommonsAttribution3_0Unported,
    #[serde(rename = "cc-by-3.0-at")]
    CreativeCommonsAttribution3_0Austria,
    #[serde(rename = "cc-by-3.0-de")]
    CreativeCommonsAttribution3_0Germany,
    #[serde(rename = "cc-by-3.0-igo")]
    CreativeCommonsAttribution3_0IGO,
    #[serde(rename = "cc-by-3.0-nl")]
    CreativeCommonsAttribution3_0Netherlands,
    #[serde(rename = "cc-by-3.0-us")]
    CreativeCommonsAttribution3_0UnitedStates,
    #[serde(rename = "cc-by-4.0")]
    CreativeCommonsAttribution4_0International,
    #[serde(rename = "cc-by-nc-1.0")]
    CreativeCommonsAttributionNonCommercial1_0Generic,
    #[serde(rename = "cc-by-nc-2.0")]
    CreativeCommonsAttributionNonCommercial2_0Generic,
    #[serde(rename = "cc-by-nc-2.5")]
    CreativeCommonsAttributionNonCommercial2_5Generic,
    #[serde(rename = "cc-by-nc-3.0")]
    CreativeCommonsAttributionNonCommercial3_0Unported,
    #[serde(rename = "cc-by-nc-3.0-de")]
    CreativeCommonsAttributionNonCommercial3_0Germany,
    #[serde(rename = "cc-by-nc-4.0")]
    CreativeCommonsAttributionNonCommercial4_0International,
    #[serde(rename = "cc-by-nc-nd-1.0")]
    CreativeCommonsAttributionNonCommercialNoDerivatives1_0Generic,
    #[serde(rename = "cc-by-nc-nd-2.0")]
    CreativeCommonsAttributionNonCommercialNoDerivatives2_0Generic,
    #[serde(rename = "cc-by-nc-nd-2.5")]
    CreativeCommonsAttributionNonCommercialNoDerivatives2_5Generic,
    #[serde(rename = "cc-by-nc-nd-3.0")]
    CreativeCommonsAttributionNonCommercialNoDerivatives3_0Unported,
    #[serde(rename = "cc-by-nc-nd-3.0-de")]
    CreativeCommonsAttributionNonCommercialNoDerivatives3_0Germany,
    #[serde(rename = "cc-by-nc-nd-3.0-igo")]
    CreativeCommonsAttributionNonCommercialNoDerivatives3_0IGO,
    #[serde(rename = "cc-by-nc-nd-4.0")]
    CreativeCommonsAttributionNonCommercialNoDerivatives4_0International,
    #[serde(rename = "cc-by-nc-sa-1.0")]
    CreativeCommonsAttributionNonCommercialShareAlike1_0Generic,
    #[serde(rename = "cc-by-nc-sa-2.0")]
    CreativeCommonsAttributionNonCommercialShareAlike2_0Generic,
    #[serde(rename = "cc-by-nc-sa-2.0-fr")]
    CreativeCommonsAttributionNonCommercialShareAlike2_0France,
    #[serde(rename = "cc-by-nc-sa-2.0-uk")]
    CreativeCommonsAttributionNonCommercialShareAlike2_0EnglandandWales,
    #[serde(rename = "cc-by-nc-sa-2.5")]
    CreativeCommonsAttributionNonCommercialShareAlike2_5Generic,
    #[serde(rename = "cc-by-nc-sa-3.0")]
    CreativeCommonsAttributionNonCommercialShareAlike3_0Unported,
    #[serde(rename = "cc-by-nc-sa-3.0-de")]
    CreativeCommonsAttributionNonCommercialShareAlike3_0Germany,
    #[serde(rename = "cc-by-nc-sa-3.0-igo")]
    CreativeCommonsAttributionNonCommercialShareAlike3_0IGO,
    #[serde(rename = "cc-by-nc-sa-4.0")]
    CreativeCommonsAttributionNonCommercialShareAlike4_0International,
    #[serde(rename = "cc-by-nd-1.0")]
    CreativeCommonsAttributionNoDerivatives1_0Generic,
    #[serde(rename = "cc-by-nd-2.0")]
    CreativeCommonsAttributionNoDerivatives2_0Generic,
    #[serde(rename = "cc-by-nd-2.5")]
    CreativeCommonsAttributionNoDerivatives2_5Generic,
    #[serde(rename = "cc-by-nd-3.0")]
    CreativeCommonsAttributionNoDerivatives3_0Unported,
    #[serde(rename = "cc-by-nd-3.0-de")]
    CreativeCommonsAttributionNoDerivatives3_0Germany,
    #[serde(rename = "cc-by-nd-4.0")]
    CreativeCommonsAttributionNoDerivatives4_0International,
    #[serde(rename = "cc-by-sa-1.0")]
    CreativeCommonsAttributionShareAlike1_0Generic,
    #[serde(rename = "cc-by-sa-2.0")]
    CreativeCommonsAttributionShareAlike2_0Generic,
    #[serde(rename = "cc-by-sa-2.0-uk")]
    CreativeCommonsAttributionShareAlike2_0EnglandandWales,
    #[serde(rename = "cc-by-sa-2.1-jp")]
    CreativeCommonsAttributionShareAlike2_1Japan,
    #[serde(rename = "cc-by-sa-2.5")]
    CreativeCommonsAttributionShareAlike2_5Generic,
    #[serde(rename = "cc-by-sa-3.0")]
    CreativeCommonsAttributionShareAlike3_0Unported,
    #[serde(rename = "cc-by-sa-3.0-at")]
    CreativeCommonsAttributionShareAlike3_0Austria,
    #[serde(rename = "cc-by-sa-3.0-de")]
    CreativeCommonsAttributionShareAlike3_0Germany,
    #[serde(rename = "cc-by-sa-4.0")]
    CreativeCommonsAttributionShareAlike4_0International,
    #[serde(rename = "cc-pddc")]
    CreativeCommonsPublicDomainDedicationandCertification,
    #[serde(rename = "cc0-1.0")]
    CreativeCommonsZerov1_0Universal,
    #[serde(rename = "cddl-1.0")]
    CommonDevelopmentandDistributionLicense1_0,
    #[serde(rename = "cddl-1.1")]
    CommonDevelopmentandDistributionLicense1_1,
    #[serde(rename = "cdl-1.0")]
    CommonDocumentationLicense1_0,
    #[serde(rename = "cdla-permissive-1.0")]
    CommunityDataLicenseAgreementPermissive1_0,
    #[serde(rename = "cdla-permissive-2.0")]
    CommunityDataLicenseAgreementPermissive2_0,
    #[serde(rename = "cdla-sharing-1.0")]
    CommunityDataLicenseAgreementSharing1_0,
    #[serde(rename = "cecill-1.0")]
    CeCILLFreeSoftwareLicenseAgreementv1_0,
    #[serde(rename = "cecill-1.1")]
    CeCILLFreeSoftwareLicenseAgreementv1_1,
    #[serde(rename = "cecill-2.0")]
    CeCILLFreeSoftwareLicenseAgreementv2_0,
    #[serde(rename = "cecill-2.1")]
    CeCILLFreeSoftwareLicenseAgreementv2_1,
    #[serde(rename = "cecill-b")]
    CeCILLBFreeSoftwareLicenseAgreement,
    #[serde(rename = "cecill-c")]
    CeCILLCFreeSoftwareLicenseAgreement,
    #[serde(rename = "cern-ohl-1.1")]
    CERNOpenHardwareLicencev1_1,
    #[serde(rename = "cern-ohl-1.2")]
    CERNOpenHardwareLicencev1_2,
    #[serde(rename = "cern-ohl-p-2.0")]
    CERNOpenHardwareLicenceVersion2Permissive,
    #[serde(rename = "cern-ohl-s-2.0")]
    CERNOpenHardwareLicenceVersion2StronglyReciprocal,
    #[serde(rename = "cern-ohl-w-2.0")]
    CERNOpenHardwareLicenceVersion2WeaklyReciprocal,
    #[serde(rename = "clartistic")]
    ClarifiedArtisticLicense,
    #[serde(rename = "cnri-jython")]
    CNRIJythonLicense,
    #[serde(rename = "cnri-python")]
    CNRIPythonLicense,
    #[serde(rename = "cnri-python-gpl-compatible")]
    CNRIPythonOpenSourceGPLCompatibleLicenseAgreement,
    #[serde(rename = "coil-1.0")]
    CopyfreeOpenInnovationLicense,
    #[serde(rename = "community-spec-1.0")]
    CommunitySpecificationLicense1_0,
    #[serde(rename = "condor-1.1")]
    CondorPublicLicensev1_1,
    #[serde(rename = "copyleft-next-0.3.0")]
    Copyleftnext0_3_0,
    #[serde(rename = "copyleft-next-0.3.1")]
    Copyleftnext0_3_1,
    #[serde(rename = "cpal-1.0")]
    CommonPublicAttributionLicense1_0,
    #[serde(rename = "cpl-1.0")]
    CommonPublicLicense1_0,
    #[serde(rename = "cpol-1.02")]
    CodeProjectOpenLicense1_02,
    #[serde(rename = "crossword")]
    CrosswordLicense,
    #[serde(rename = "crystalstacker")]
    CrystalStackerLicense,
    #[serde(rename = "cua-opl-1.0")]
    CUAOfficePublicLicensev1_0,
    #[serde(rename = "cube")]
    CubeLicense,
    #[serde(rename = "curl")]
    CurlLicense,
    #[serde(rename = "d-fsl-1.0")]
    DeutscheFreieSoftwareLizenz,
    #[serde(rename = "diffmark")]
    DiffmarkLicense,
    #[serde(rename = "dl-de-by-2.0")]
    DatalicenceGermanyAttributionVersion2_0,
    #[serde(rename = "doc")]
    DOCLicense,
    #[serde(rename = "dotseqn")]
    DotseqnLicense,
    #[serde(rename = "drl-1.0")]
    DetectionRuleLicense1_0,
    #[serde(rename = "dsdp")]
    DSDPLicense,
    #[serde(rename = "dvipdfm")]
    DvipdfmLicense,
    #[serde(rename = "ecl-1.0")]
    EducationalCommunityLicensev1_0,
    #[serde(rename = "ecl-2.0")]
    EducationalCommunityLicensev2_0,
    #[serde(rename = "efl-1.0")]
    EiffelForumLicensev1_0,
    #[serde(rename = "efl-2.0")]
    EiffelForumLicensev2_0,
    #[serde(rename = "egenix")]
    EGenixComPublicLicense1_1_0,
    #[serde(rename = "elastic-2.0")]
    ElasticLicense2_0,
    #[serde(rename = "entessa")]
    EntessaPublicLicensev1_0,
    #[serde(rename = "epics")]
    EPICSOpenLicense,
    #[serde(rename = "epl-1.0")]
    EclipsePublicLicense1_0,
    #[serde(rename = "epl-2.0")]
    EclipsePublicLicense2_0,
    #[serde(rename = "erlpl-1.1")]
    ErlangPublicLicensev1_1,
    #[serde(rename = "etalab-2.0")]
    EtalabOpenLicense2_0,
    #[serde(rename = "eudatagrid")]
    EUDataGridSoftwareLicense,
    #[serde(rename = "eupl-1.0")]
    EuropeanUnionPublicLicense1_0,
    #[serde(rename = "eupl-1.1")]
    EuropeanUnionPublicLicense1_1,
    #[serde(rename = "eupl-1.2")]
    EuropeanUnionPublicLicense1_2,
    #[serde(rename = "eurosym")]
    EurosymLicense,
    #[serde(rename = "fair")]
    FairLicense,
    #[serde(rename = "fdk-aac")]
    FraunhoferFDKAACCodecLibrary,
    #[serde(rename = "frameworx-1.0")]
    FrameworxOpenLicense1_0,
    #[serde(rename = "freebsd-doc")]
    FreeBSDDocumentationLicense,
    #[serde(rename = "freeimage")]
    FreeImagePublicLicensev1_0,
    #[serde(rename = "fsfap")]
    FSFAllPermissiveLicense,
    #[serde(rename = "fsful")]
    FSFUnlimitedLicense,
    #[serde(rename = "fsfullr")]
    FSFUnlimitedLicenseWithLicenseRetention,
    #[serde(rename = "ftl")]
    FreetypeProjectLicense,
    #[serde(rename = "gd")]
    GDLicense,
    #[serde(rename = "gfdl-1.1-invariants-only")]
    GNUFreeDocumentationLicensev1_1onlyinvariants,
    #[serde(rename = "gfdl-1.1-invariants-or-later")]
    GNUFreeDocumentationLicensev1_1orlaterinvariants,
    #[serde(rename = "gfdl-1.1-no-invariants-only")]
    GNUFreeDocumentationLicensev1_1onlynoinvariants,
    #[serde(rename = "gfdl-1.1-no-invariants-or-later")]
    GNUFreeDocumentationLicensev1_1orlaternoinvariants,
    #[serde(rename = "gfdl-1.1-only")]
    GNUFreeDocumentationLicensev1_1only,
    #[serde(rename = "gfdl-1.1-or-later")]
    GNUFreeDocumentationLicensev1_1orlater,
    #[serde(rename = "gfdl-1.2-invariants-only")]
    GNUFreeDocumentationLicensev1_2onlyinvariants,
    #[serde(rename = "gfdl-1.2-invariants-or-later")]
    GNUFreeDocumentationLicensev1_2orlaterinvariants,
    #[serde(rename = "gfdl-1.2-no-invariants-only")]
    GNUFreeDocumentationLicensev1_2onlynoinvariants,
    #[serde(rename = "gfdl-1.2-no-invariants-or-later")]
    GNUFreeDocumentationLicensev1_2orlaternoinvariants,
    #[serde(rename = "gfdl-1.2-only")]
    GNUFreeDocumentationLicensev1_2only,
    #[serde(rename = "gfdl-1.2-or-later")]
    GNUFreeDocumentationLicensev1_2orlater,
    #[serde(rename = "gfdl-1.3-invariants-only")]
    GNUFreeDocumentationLicensev1_3onlyinvariants,
    #[serde(rename = "gfdl-1.3-invariants-or-later")]
    GNUFreeDocumentationLicensev1_3orlaterinvariants,
    #[serde(rename = "gfdl-1.3-no-invariants-only")]
    GNUFreeDocumentationLicensev1_3onlynoinvariants,
    #[serde(rename = "gfdl-1.3-no-invariants-or-later")]
    GNUFreeDocumentationLicensev1_3orlaternoinvariants,
    #[serde(rename = "gfdl-1.3-only")]
    GNUFreeDocumentationLicensev1_3only,
    #[serde(rename = "gfdl-1.3-or-later")]
    GNUFreeDocumentationLicensev1_3orlater,
    #[serde(rename = "giftware")]
    GiftwareLicense,
    #[serde(rename = "gl2ps")]
    GL2PSLicense,
    #[serde(rename = "glide")]
    DfxGlideLicense,
    #[serde(rename = "glulxe")]
    GlulxeLicense,
    #[serde(rename = "glwtpl")]
    GoodLuckWithThatPublicLicense,
    #[serde(rename = "gnuplot")]
    GnuPlotLicense,
    #[serde(rename = "gpl-1.0-only")]
    GNUGeneralPublicLicensev1_0only,
    #[serde(rename = "gpl-1.0-or-later")]
    GNUGeneralPublicLicensev1_0orlater,
    #[serde(rename = "gpl-2.0-only")]
    GNUGeneralPublicLicensev2_0only,
    #[serde(rename = "gpl-2.0-or-later")]
    GNUGeneralPublicLicensev2_0orlater,
    #[serde(rename = "gpl-3.0-only")]
    GNUGeneralPublicLicensev3_0only,
    #[serde(rename = "gpl-3.0-or-later")]
    GNUGeneralPublicLicensev3_0orlater,
    #[serde(rename = "gsoap-1.3b")]
    GSOAPPublicLicensev1_3b,
    #[serde(rename = "haskellreport")]
    HaskellLanguageReportLicense,
    #[serde(rename = "hippocratic-2.1")]
    HippocraticLicense2_1,
    #[serde(rename = "hpnd")]
    HistoricalPermissionNoticeandDisclaimer,
    #[serde(rename = "hpnd-sell-variant")]
    HistoricalPermissionNoticeandDisclaimersellvariant,
    #[serde(rename = "htmltidy")]
    HTMLTidyLicense,
    #[serde(rename = "ibm-pibs")]
    IBMPowerPCInitializationandBootSoftware,
    #[serde(rename = "icu")]
    ICULicense,
    #[serde(rename = "ijg")]
    IndependentJPEGGroupLicense,
    #[serde(rename = "imagemagick")]
    ImageMagickLicense,
    #[serde(rename = "imatix")]
    IMatixStandardFunctionLibraryAgreement,
    #[serde(rename = "imlib2")]
    Imlib2License,
    #[serde(rename = "info-zip")]
    InfoZIPLicense,
    #[serde(rename = "intel")]
    IntelOpenSourceLicense,
    #[serde(rename = "intel-acpi")]
    IntelACPISoftwareLicenseAgreement,
    #[serde(rename = "interbase-1.0")]
    InterbasePublicLicensev1_0,
    #[serde(rename = "ipa")]
    IPAFontLicense,
    #[serde(rename = "ipl-1.0")]
    IBMPublicLicensev1_0,
    #[serde(rename = "isc")]
    ISCLicense,
    #[serde(rename = "jam")]
    JamLicense,
    #[serde(rename = "jasper-2.0")]
    JasPerLicense,
    #[serde(rename = "jpnic")]
    JapanNetworkInformationCenterLicense,
    #[serde(rename = "json")]
    JSONLicense,
    #[serde(rename = "lal-1.2")]
    LicenceArtLibre1_2,
    #[serde(rename = "lal-1.3")]
    LicenceArtLibre1_3,
    #[serde(rename = "latex2e")]
    Latex2eLicense,
    #[serde(rename = "leptonica")]
    LeptonicaLicense,
    #[serde(rename = "lgpl-2.0-only")]
    GNULibraryGeneralPublicLicensev2only,
    #[serde(rename = "lgpl-2.0-or-later")]
    GNULibraryGeneralPublicLicensev2orlater,
    #[serde(rename = "lgpl-2.1-only")]
    GNULesserGeneralPublicLicensev2_1only,
    #[serde(rename = "lgpl-2.1-or-later")]
    GNULesserGeneralPublicLicensev2_1orlater,
    #[serde(rename = "lgpl-3.0-only")]
    GNULesserGeneralPublicLicensev3_0only,
    #[serde(rename = "lgpl-3.0-or-later")]
    GNULesserGeneralPublicLicensev3_0orlater,
    #[serde(rename = "lgpllr")]
    LesserGeneralPublicLicenseForLinguisticResources,
    #[serde(rename = "libpng")]
    LibpngLicense,
    #[serde(rename = "libpng-2.0")]
    PNGReferenceLibraryversion2,
    #[serde(rename = "libselinux-1.0")]
    LibselinuxPublicDomaiNnotice,
    #[serde(rename = "libtiff")]
    LibtiffLicense,
    #[serde(rename = "liliq-p-1.1")]
    LicenceLibreduQuebecPermissiveversion1_1,
    #[serde(rename = "liliq-r-1.1")]
    LicenceLibreduQuebecReciprociteversion1_1,
    #[serde(rename = "liliq-rplus-1.1")]
    LicenceLibreduQuebecReciprociteforteversion1_1,
    #[serde(rename = "linux-man-pages-copyleft")]
    LinuxmanpagesCopyleft,
    #[serde(rename = "linux-openib")]
    LinuxKernelVariantofOpenIBOrgLicense,
    #[serde(rename = "lpl-1.0")]
    LucentPublicLicenseVersion1_0,
    #[serde(rename = "lpl-1.02")]
    LucentPublicLicensev1_02,
    #[serde(rename = "lppl-1.0")]
    LaTeXProjectPublicLicensev1_0,
    #[serde(rename = "lppl-1.1")]
    LaTeXProjectPublicLicensev1_1,
    #[serde(rename = "lppl-1.2")]
    LaTeXProjectPublicLicensev1_2,
    #[serde(rename = "lppl-1.3a")]
    LaTeXProjectPublicLicensev1_3a,
    #[serde(rename = "lppl-1.3c")]
    LaTeXProjectPublicLicensev1_3c,
    #[serde(rename = "lzma-sdk-9.11-to-9.20")]
    LZMASDKLicenseVersions9_11To9_20,
    #[serde(rename = "lzma-sdk-9.22")]
    LZMASDKLicenseVersions9_22AndBeyond,
    #[serde(rename = "makeindex")]
    MakeIndexLicense,
    #[serde(rename = "minpack")]
    MinpackLicense,
    #[serde(rename = "miros")]
    TheMirOSLicence,
    #[serde(rename = "mit")]
    MITLicense,
    #[serde(rename = "mit-0")]
    MITNoAttribution,
    #[serde(rename = "mit-advertising")]
    EnlightenmentLicenseE16,
    #[serde(rename = "mit-cmu")]
    CMULicense,
    #[serde(rename = "mit-enna")]
    EnnaLicense,
    #[serde(rename = "mit-feh")]
    FehLicense,
    #[serde(rename = "mit-modern-variant")]
    MITLicenseModernVariant,
    #[serde(rename = "mit-open-group")]
    MITOpenGroupvariant,
    #[serde(rename = "mitnfa")]
    MITNoFalseAttribsLicense,
    #[serde(rename = "motosoto")]
    MotosotoLicense,
    #[serde(rename = "mpi-permissive")]
    MpiPermissiveLicense,
    #[serde(rename = "mpich2")]
    Mpich2License,
    #[serde(rename = "mpl-1.0")]
    MozillaPublicLicense1_0,
    #[serde(rename = "mpl-1.1")]
    MozillaPublicLicense1_1,
    #[serde(rename = "mpl-2.0")]
    MozillaPublicLicense2_0,
    #[serde(rename = "mpl-2.0-no-copyleft-exception")]
    MozillaPublicLicense2_0nocopyleftexception,
    #[serde(rename = "mplus")]
    MplusFontLicense,
    #[serde(rename = "ms-lpl")]
    MicrosoftLimitedPublicLicense,
    #[serde(rename = "ms-pl")]
    MicrosoftPublicLicense,
    #[serde(rename = "ms-rl")]
    MicrosoftReciprocalLicense,
    #[serde(rename = "mtll")]
    MatrixTemplateLibraryLicense,
    #[serde(rename = "mulanpsl-1.0")]
    MulanPermissiveSoftwareLicenseVersion1,
    #[serde(rename = "mulanpsl-2.0")]
    MulanPermissiveSoftwareLicenseVersion2,
    #[serde(rename = "multics")]
    MulticsLicense,
    #[serde(rename = "mup")]
    MupLicense,
    #[serde(rename = "naist-2003")]
    NaraInstituteofScienceandTechnologyLicense2003,
    #[serde(rename = "nasa-1.3")]
    NASAOpenSourceAgreement1_3,
    #[serde(rename = "naumen")]
    NaumenPublicLicense,
    #[serde(rename = "nbpl-1.0")]
    NetBooleanPublicLicensev1,
    #[serde(rename = "ncgl-uk-2.0")]
    NonCommercialGovernmentLicence,
    #[serde(rename = "ncsa")]
    UniversityofIllinoisNCSAOpenSourceLicense,
    #[serde(rename = "net-snmp")]
    NetSNMPLicense,
    #[serde(rename = "netcdf")]
    NetCDFlicense,
    #[serde(rename = "newsletr")]
    NewsletrLicense,
    #[serde(rename = "ngpl")]
    NethackGeneralPublicLicense,
    #[serde(rename = "nicta-1.0")]
    NICTAPublicSoftwareLicense,
    Version1_0,
    #[serde(rename = "nist-pd")]
    NISTPublicDomainNotice,
    #[serde(rename = "nist-pd-fallback")]
    NISTPublicDomainNoticewithlicensefallback,
    #[serde(rename = "nlod-1.0")]
    NorwegianLicenceforOpenGovernmentDataNLOD1_0,
    #[serde(rename = "nlod-2.0")]
    NorwegianLicenceforOpenGovernmentDataNLOD2_0,
    #[serde(rename = "nlpl")]
    NoLimitPublicLicense,
    #[serde(rename = "nokia")]
    NokiaOpenSourceLicense,
    #[serde(rename = "nosl")]
    NetizenOpenSourceLicense,
    #[serde(rename = "noweb")]
    NowebLicense,
    #[serde(rename = "npl-1.0")]
    NetscapePublicLicensev1_0,
    #[serde(rename = "npl-1.1")]
    NetscapePublicLicensev1_1,
    #[serde(rename = "nposl-3.0")]
    NonProfitOpenSoftwareLicense3_0,
    #[serde(rename = "nrl")]
    NRLLicense,
    #[serde(rename = "ntp")]
    NTPLicense,
    #[serde(rename = "ntp-0")]
    NTPNoAttribution,
    #[serde(rename = "o-uda-1.0")]
    OpenUseofDataAgreementv1_0,
    #[serde(rename = "occt-pl")]
    OpenCASCADETechnologyPublicLicense,
    #[serde(rename = "oclc-2.0")]
    OCLCResearchPublicLicense2_0,
    #[serde(rename = "odbl-1.0")]
    OpenDataCommonsOpenDatabaseLicensev1_0,
    #[serde(rename = "odc-by-1.0")]
    OpenDataCommonsAttributionLicensev1_0,
    #[serde(rename = "ofl-1.0")]
    SILOpenFontLicense1_0,
    #[serde(rename = "ofl-1.0-no-rfn")]
    SILOpenFontLicense1_0withnoReservedFontName,
    #[serde(rename = "ofl-1.0-rfn")]
    SILOpenFontLicense1_0withReservedFontName,
    #[serde(rename = "ofl-1.1")]
    SILOpenFontLicense1_1,
    #[serde(rename = "ofl-1.1-no-rfn")]
    SILOpenFontLicense1_1withnoReservedFontName,
    #[serde(rename = "ofl-1.1-rfn")]
    SILOpenFontLicense1_1withReservedFontName,
    #[serde(rename = "ogc-1.0")]
    OGCSoftwareLicenseVersion1_0,
    #[serde(rename = "ogdl-taiwan-1.0")]
    TaiwanOpenGovernmentDataLicenseVersion1_0,
    #[serde(rename = "ogl-canada-2.0")]
    OpenGovernmentLicenceCanada,
    #[serde(rename = "ogl-uk-1.0")]
    OpenGovernmentLicencev1_0,
    #[serde(rename = "ogl-uk-2.0")]
    OpenGovernmentLicencev2_0,
    #[serde(rename = "ogl-uk-3.0")]
    OpenGovernmentLicencev3_0,
    #[serde(rename = "ogtsl")]
    OpenGroupTestSuiteLicense,
    #[serde(rename = "oldap-1.1")]
    OpenLDAPPublicLicensev1_1,
    #[serde(rename = "oldap-1.2")]
    OpenLDAPPublicLicensev1_2,
    #[serde(rename = "oldap-1.3")]
    OpenLDAPPublicLicensev1_3,
    #[serde(rename = "oldap-1.4")]
    OpenLDAPPublicLicensev1_4,
    #[serde(rename = "oldap-2.0")]
    OpenLDAPPublicLicensev2_0Orpossibly2_0Aand2_0B,
    #[serde(rename = "oldap-2.0.1")]
    OpenLDAPPublicLicensev2_0_1,
    #[serde(rename = "oldap-2.1")]
    OpenLDAPPublicLicensev2_1,
    #[serde(rename = "oldap-2.2")]
    OpenLDAPPublicLicensev2_2,
    #[serde(rename = "oldap-2.2.1")]
    OpenLDAPPublicLicensev2_2_1,
    #[serde(rename = "oldap-2.2.2")]
    OpenLDAPPublicLicense2_2_2,
    #[serde(rename = "oldap-2.3")]
    OpenLDAPPublicLicensev2_3,
    #[serde(rename = "oldap-2.4")]
    OpenLDAPPublicLicensev2_4,
    #[serde(rename = "oldap-2.5")]
    OpenLDAPPublicLicensev2_5,
    #[serde(rename = "oldap-2.6")]
    OpenLDAPPublicLicensev2_6,
    #[serde(rename = "oldap-2.7")]
    OpenLDAPPublicLicensev2_7,
    #[serde(rename = "oldap-2.8")]
    OpenLDAPPublicLicensev2_8,
    #[serde(rename = "oml")]
    OpenMarketLicense,
    #[serde(rename = "openssl")]
    OpenSSLLicense,
    #[serde(rename = "opl-1.0")]
    OpenPublicLicensev1_0,
    #[serde(rename = "opubl-1.0")]
    OpenPublicationLicensev1_0,
    #[serde(rename = "oset-pl-2.1")]
    OSETPublicLicenseversion2_1,
    #[serde(rename = "osl-1.0")]
    OpenSoftwareLicense1_0,
    #[serde(rename = "osl-1.1")]
    OpenSoftwareLicense1_1,
    #[serde(rename = "osl-2.0")]
    OpenSoftwareLicense2_0,
    #[serde(rename = "osl-2.1")]
    OpenSoftwareLicense2_1,
    #[serde(rename = "osl-3.0")]
    OpenSoftwareLicense3_0,
    #[serde(rename = "parity-6.0.0")]
    TheParityPublicLicense6_0_0,
    #[serde(rename = "parity-7.0.0")]
    TheParityPublicLicense7_0_0,
    #[serde(rename = "pddl-1.0")]
    OpenDataCommonsPublicDomainDedicationLicense1_0,
    #[serde(rename = "php-3.0")]
    PHPLicensev3_0,
    #[serde(rename = "php-3.01")]
    PHPLicensev3_01,
    #[serde(rename = "plexus")]
    PlexusClassworldsLicense,
    #[serde(rename = "polyform-noncommercial-1.0.0")]
    PolyFormNoncommercialLicense1_0_0,
    #[serde(rename = "polyform-small-business-1.0.0")]
    PolyFormSmallBusinessLicense1_0_0,
    #[serde(rename = "postgresql")]
    PostgreSQLLicense,
    #[serde(rename = "psf-2.0")]
    PythonSoftwareFoundationLicense2_0,
    #[serde(rename = "psfrag")]
    PsfragLicense,
    #[serde(rename = "psutils")]
    PsutilsLicense,
    #[serde(rename = "python-2.0")]
    PythonLicense2_0,
    #[serde(rename = "python-2.0.1")]
    PythonLicense2_0_1,
    #[serde(rename = "qhull")]
    QhullLicense,
    #[serde(rename = "qpl-1.0")]
    QPublicLicense1_0,
    #[serde(rename = "rdisc")]
    RdiscLicense,
    #[serde(rename = "rhecos-1.1")]
    RedHateCosPublicLicensev1_1,
    #[serde(rename = "rpl-1.1")]
    ReciprocalPublicLicense1_1,
    #[serde(rename = "rpl-1.5")]
    ReciprocalPublicLicense1_5,
    #[serde(rename = "rpsl-1.0")]
    RealNetworksPublicSourceLicensev1_0,
    #[serde(rename = "rsa-md")]
    RSAMessageDigestLicense,
    #[serde(rename = "rscpl")]
    RicohSourceCodePublicLicense,
    #[serde(rename = "ruby")]
    RubyLicense,
    #[serde(rename = "sax-pd")]
    SaxPublicDomainNotice,
    #[serde(rename = "saxpath")]
    SaxpathLicense,
    #[serde(rename = "scea")]
    SCEASharedSourceLicense,
    #[serde(rename = "schemereport")]
    SchemeLanguageReportLicense,
    #[serde(rename = "sendmail")]
    SendmailLicense,
    #[serde(rename = "sendmail-8.23")]
    SendmailLicense8_23,
    #[serde(rename = "sgi-b-1.0")]
    SGIFreeSoftwareLicenseBv1_0,
    #[serde(rename = "sgi-b-1.1")]
    SGIFreeSoftwareLicenseBv1_1,
    #[serde(rename = "sgi-b-2.0")]
    SGIFreeSoftwareLicenseBv2_0,
    #[serde(rename = "shl-0.5")]
    SolderpadHardwareLicensev0_5,
    #[serde(rename = "shl-0.51")]
    SolderpadHardwareLicense,
    Version0_51,
    #[serde(rename = "simpl-2.0")]
    SimplePublicLicense2_0,
    #[serde(rename = "sissl")]
    SunIndustryStandardsSourceLicensev1_1,
    #[serde(rename = "sissl-1.2")]
    SunIndustryStandardsSourceLicensev1_2,
    #[serde(rename = "sleepycat")]
    SleepycatLicense,
    #[serde(rename = "smlnj")]
    StandardMLofNewJerseyLicense,
    #[serde(rename = "smppl")]
    SecureMessagingProtocolPublicLicense,
    #[serde(rename = "snia")]
    SNIAPublicLicense1_1,
    #[serde(rename = "spencer-86")]
    SpencerLicense86,
    #[serde(rename = "spencer-94")]
    SpencerLicense94,
    #[serde(rename = "spencer-99")]
    SpencerLicense99,
    #[serde(rename = "spl-1.0")]
    SunPublicLicensev1_0,
    #[serde(rename = "ssh-openssh")]
    SSHOpenSSHlicense,
    #[serde(rename = "ssh-short")]
    SSHshortnotice,
    #[serde(rename = "sspl-1.0")]
    ServerSidePublicLicensev1,
    #[serde(rename = "sugarcrm-1.1.3")]
    SugarCRMPublicLicensev1_1_3,
    #[serde(rename = "swl")]
    SchemeWidgetLibrarySWLSoftwareLicenseAgreement,
    #[serde(rename = "tapr-ohl-1.0")]
    TAPROpenHardwareLicensev1_0,
    #[serde(rename = "tcl")]
    TCLTKLicense,
    #[serde(rename = "tcp-wrappers")]
    TCPWrappersLicense,
    #[serde(rename = "tmate")]
    TMateOpenSourceLicense,
    #[serde(rename = "torque-1.1")]
    TORQUEv2_5SoftwareLicensev1_1,
    #[serde(rename = "tosl")]
    TrussterOpenSourceLicense,
    #[serde(rename = "tu-berlin-1.0")]
    TechnischeUniversitaetBerlinLicense1_0,
    #[serde(rename = "tu-berlin-2.0")]
    TechnischeUniversitaetBerlinLicense2_0,
    #[serde(rename = "ucl-1.0")]
    UpstreamCompatibilityLicensev1_0,
    #[serde(rename = "unicode-dfs-2015")]
    UnicodeLicenseAgreementDataFilesandSoftware2015,
    #[serde(rename = "unicode-dfs-2016")]
    UnicodeLicenseAgreementDataFilesandSoftware2016,
    #[serde(rename = "unicode-tou")]
    UnicodeTermsofUse,
    #[serde(rename = "unlicense")]
    TheUnlicense,
    #[serde(rename = "upl-1.0")]
    UniversalPermissiveLicensev1_0,
    #[serde(rename = "vim")]
    VimLicense,
    #[serde(rename = "vostrom")]
    VOSTROMPublicLicenseforOpenSource,
    #[serde(rename = "vsl-1.0")]
    VovidaSoftwareLicensev1_0,
    #[serde(rename = "w3c")]
    W3CSoftwareNoticeandLicense20021231,
    #[serde(rename = "w3c-19980720")]
    W3CSoftwareNoticeandLicense19980720,
    #[serde(rename = "w3c-20150513")]
    W3CSoftwareNoticeandDocumentLicense20150513,
    #[serde(rename = "watcom-1.0")]
    SybaseOpenWatcomPublicLicense1_0,
    #[serde(rename = "wsuipa")]
    WsuipaLicense,
    #[serde(rename = "wtfpl")]
    DoWhatTheFckYouWantToPublicLicense,
    #[serde(rename = "x11")]
    X11License,
    #[serde(rename = "x11-distribute-modifications-variant")]
    X11LicenseDistributionModificationVariant,
    #[serde(rename = "xerox")]
    XeroxLicense,
    #[serde(rename = "xfree86-1.1")]
    XFree86License1_1,
    #[serde(rename = "xinetd")]
    XinetdLicense,
    #[serde(rename = "xnet")]
    XNetLicense,
    #[serde(rename = "xpp")]
    XPPLicense,
    #[serde(rename = "xskat")]
    XSkatLicense,
    #[serde(rename = "ypl-1.0")]
    YahooPublicLicensev1_0,
    #[serde(rename = "ypl-1.1")]
    YahooPublicLicensev1_1,
    #[serde(rename = "zed")]
    ZedLicense,
    #[serde(rename = "zend-2.0")]
    ZendLicensev2_0,
    #[serde(rename = "zimbra-1.3")]
    ZimbraPublicLicensev1_3,
    #[serde(rename = "zimbra-1.4")]
    ZimbraPublicLicensev1_4,
    #[serde(rename = "zlib")]
    ZlibLicense,
    #[serde(rename = "zlib-acknowledgement")]
    ZlibLibpngLicensewithAcknowledgement,
    #[serde(rename = "zpl-1.1")]
    ZopePublicLicense1_1,
    #[serde(rename = "zpl-2.0")]
    ZopePublicLicense2_0,
    #[serde(rename = "zpl-2.1")]
    ZopePublicLicense2_1,

    #[serde(other)]
    Other(String),
}
